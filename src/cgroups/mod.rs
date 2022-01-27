pub mod common;
pub mod v1;
pub mod v2;

use anyhow::Result;
use common::ControllerOpt;
use nix::unistd::Pid;

pub const SMOG: &str = "smog";

pub const DEFAULT_CGROUP_PATH: &str = "/sys/fs/cgroup";

pub enum CgroupVersion {
    V1,
    V2,
}

pub trait CgroupManager {
    fn add_task(&self, pid: Pid) -> Result<()>;
    fn apply(&self, controller_opt: &ControllerOpt) -> Result<()>;
}
