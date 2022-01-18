use super::subsystem::{SubSystemType, SUBSYSTEMLIST};
use crate::utils::fs;
use anyhow::{anyhow, bail, Result};
use procfs::process::Process;

use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

const root: &str = "smog";

pub struct CgroupsManager {
    subsystems: HashMap<SubSystemType, PathBuf>,
}

impl CgroupsManager {
    pub fn new(container_id: &str) -> CgroupsManager {
        let cgroups_path = PathBuf::from(format!("/{}/{}", root, container_id));
        for subsystem in SUBSYSTEMLIST {
            Self::get_subsystem_path(&cgroups_path, &subsystem);
        }
        Self {
            subsystems: HashMap::new(),
        }
    }

    fn add_task() {}

    fn get_subsystem_path(path: &Path, subsystem: &SubSystemType) -> Result<PathBuf> {
        let mount_point = get_subsystem_mount_point(subsystem).unwrap();
        let p = mount_point.join(path);
        Ok(p)
    }
}

// 当要向某个 CGroup 加入 Thread 时，将Thread PID 写入 tasks 或 cgroup.procs 即可，
// cgroup.procs 会自动变更为该 Task 所属的 Proc PID。
// 如果要加入 Proc 时，则只能写入到 cgroup.procs 文件(未解)，tasks 文件会自动更新为该
// Proc 下所有的 Thread PID。
// 可以通过cat /proc/PID/cgroup查看某个 Proc/Thread 的 CGroup 信息
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
