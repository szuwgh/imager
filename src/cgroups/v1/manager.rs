use super::subsystem::{SubSystemType, SUBSYSTEMLIST};
use crate::utils::fs;
use anyhow::{anyhow, bail, Result};
use procfs::process::Process;

use crate::cgroups::SMOG;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

pub struct Manager {
    subsystems: HashMap<SubSystemType, PathBuf>,
}

impl Manager {
    pub fn new(container_id: &str) -> Manager {
        let mut subsystems: HashMap<SubSystemType, PathBuf> = HashMap::new();
        let cgroups_path = PathBuf::from(format!("/{}/{}", SMOG, container_id));
        for subsystem in SUBSYSTEMLIST {
            if let Ok(subsystem_path) = Self::get_subsystem_path(&cgroups_path, &subsystem) {
                subsystems.insert(subsystem.clone(), subsystem_path);
            } else {
                println!("cgroup {} not supported on this system", subsystem);
            }
        }
        Self {
            subsystems: subsystems,
        }
    }

    fn add_task() {}

    fn get_subsystem_path(path: &Path, subsystem: &SubSystemType) -> Result<PathBuf> {
        let mount_point = get_subsystem_mount_point(subsystem).unwrap();
        let p = mount_point.join(path);
        Ok(p)
    }
}

pub fn get_subsystem_mount_point(subsystem: &SubSystemType) -> Result<PathBuf> {
    let subsystem = subsystem.to_string();
    Process::myself()?
        .mountinfo()?
        .into_iter()
        .find(|m| {
            if m.fs_type == "cgroup" {
                // Some systems mount net_prio and net_cls in the same directory
                // other systems mount them in their own diretories. This
                // should handle both cases.
                if subsystem == "net_cls" {
                    return m.mount_point.ends_with("net_cls,net_prio")
                        || m.mount_point.ends_with("net_prio,net_cls")
                        || m.mount_point.ends_with("net_cls");
                } else if subsystem == "net_prio" {
                    return m.mount_point.ends_with("net_cls,net_prio")
                        || m.mount_point.ends_with("net_prio,net_cls")
                        || m.mount_point.ends_with("net_prio");
                }

                if subsystem == "cpu" {
                    return m.mount_point.ends_with("cpu,cpuacct")
                        || m.mount_point.ends_with("cpu");
                }
                if subsystem == "cpuacct" {
                    return m.mount_point.ends_with("cpu,cpuacct")
                        || m.mount_point.ends_with("cpuacct");
                }
            }
            m.mount_point.ends_with(&subsystem)
        })
        .map(|m| m.mount_point)
        .ok_or_else(|| anyhow!("could not find mountpoint for {}", subsystem))
}
