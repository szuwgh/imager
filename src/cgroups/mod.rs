pub mod v1;
pub mod v2;

use anyhow::Result;
use nix::unistd::Pid;

pub const SMOG: &str = "smog";

pub const DEFAULT_CGROUP_PATH: &str = "/sys/fs/cgroup";

pub trait CgroupManager {
    fn add_task(&self, pid: Pid) -> Result<()>;
}
