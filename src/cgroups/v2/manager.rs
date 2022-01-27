use super::cpu::Cpu;
use super::subsystem::{SubSystem, SubSystemType, SUBSYSTEMLIST};
use crate::cgroups::common;
use crate::cgroups::common::ControllerOpt;
use crate::cgroups::CgroupManager;
use crate::cgroups::SMOG;
use anyhow::Result;
use nix::unistd::Pid;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Component::RootDir;
use std::path::{Path, PathBuf};

pub const CGROUP_CONTROLLERS: &str = "cgroup.controllers";
pub const CGROUP_SUBTREE_CONTROL: &str = "cgroup.subtree_control";
pub const CGROUP_PROCS: &str = "cgroup.procs";

pub struct Manager {
    root_path: PathBuf,
    cgroups_path: PathBuf,
    full_path: PathBuf,
}

impl Manager {
    pub fn new(root_path: PathBuf, container_id: &str) -> Manager {
        let cgroups_path = PathBuf::from(format!("{}/{}", SMOG, container_id));
        let full_path = root_path.join(&cgroups_path);
        println!("full_path:{:?} ", full_path);
        Self {
            root_path: root_path,
            cgroups_path: cgroups_path,
            full_path: full_path,
        }
    }

    fn create_unified_cgroup(&self, pid: Pid) -> Result<()> {
        let controllers: Vec<String> = SUBSYSTEMLIST
            .iter()
            .map(|c| format!("{}{}", "+", c.to_string()))
            .collect();
        let mut components = self
            .cgroups_path
            .components()
            .filter(|c| c.ne(&RootDir))
            .peekable();
        let mut current_path = self.root_path.clone();
        while let Some(component) = components.next() {
            current_path = current_path.join(component);
            println!("component:{:?} , {:?}", component, current_path);
            if !current_path.exists() {
                fs::create_dir(&current_path)?;
                fs::metadata(&current_path)?.permissions().set_mode(0o755);
            }
            // (device or resource busy)
            if components.peek().is_some() {
                Self::write_controllers(&current_path, &controllers)?;
            }
        }
        common::write_cgroup_file(&self.full_path.join(CGROUP_PROCS), pid)?;
        Ok(())
    }

    fn write_controllers(path: &Path, controllers: &[String]) -> Result<()> {
        for controller in controllers {
            common::write_cgroup_file_str(path.join(CGROUP_SUBTREE_CONTROL), controller)?;
        }
        Ok(())
    }
}

impl CgroupManager for Manager {
    fn add_task(&self, pid: Pid) -> Result<()> {
        self.create_unified_cgroup(pid)?;
        Ok(())
    }

    fn apply(&self, controller_opt: &ControllerOpt) -> Result<()> {
        for controller in SUBSYSTEMLIST {
            match controller {
                SubSystemType::Cpu => Cpu::apply(controller_opt, &self.full_path)?,
                _ => {}
            }
        }
        Ok(())
    }
}
