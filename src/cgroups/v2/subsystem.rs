use crate::cgroups::common::ControllerOpt;
use anyhow::Result;
use std::fmt::Display;
use std::path::Path;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum SubSystemType {
    Cpu,
    CpuSet,
    Io,
    Memory,
    HugeTlb,
    Pids,
}

impl Display for SubSystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = match *self {
            Self::CpuSet => "cpuset",
            Self::Cpu => "cpu",
            Self::Io => "io",
            Self::Memory => "memory",
            Self::HugeTlb => "hugetlb",
            Self::Pids => "pids",
        };
        write!(f, "{}", print)
    }
}

#[allow(dead_code)]
pub const SUBSYSTEMLIST: &[SubSystemType] = &[
    SubSystemType::Cpu,
    SubSystemType::CpuSet,
    SubSystemType::Io,
    SubSystemType::Memory,
    SubSystemType::Pids,
];

pub trait SubSystem {
    fn apply(controller_opt: &ControllerOpt, cgroup_path: &Path) -> Result<()>;
}
