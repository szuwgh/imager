use std::fmt::Display;

// root@vm:~# lssubsys -a
// cpuset
// cpu
// cpuacct
// blkio
// memory
// devices
// freezer
// net_cls
// perf_event
// net_prio
// hugetlb
// pids
// rdma
// misc
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
    fn apply() {}
}
