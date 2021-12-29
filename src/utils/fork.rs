use crate::common::error::Error;
use crate::oci::oci::Namespace;
use anyhow::Result;
use nix::sched::{clone, CloneFlags};
use nix::unistd::Pid;

fn to_flags(namespace: &Namespace) -> CloneFlags {
    match namespace.typ.as_str() {
        "pid" => CloneFlags::CLONE_NEWPID,
        "network" | "net" => CloneFlags::CLONE_NEWNET,
        "mount" | "mnt" => CloneFlags::CLONE_NEWNS,
        "ipc" => CloneFlags::CLONE_NEWIPC,
        "uts" => CloneFlags::CLONE_NEWUTS,
        "user" => CloneFlags::CLONE_NEWUSER,
        "cgroup" => CloneFlags::CLONE_NEWCGROUP,
        _ => panic!("unknown namespace {}", namespace.typ),
    }
}

//克隆一个进程
pub fn clone_proc(func: impl FnMut() -> isize, namespaces: &Vec<Namespace>) -> Result<Pid> {
    const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MB
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let spec_namespaces = namespaces
        .into_iter()
        .map(|ns| to_flags(ns))
        .reduce(|a, b| a | b);

    let clone_flags = match spec_namespaces {
        Some(flags) => flags,
        None => CloneFlags::empty(),
    };
    let pid = clone(Box::new(func), stack, clone_flags, None)?;
    Ok(pid)
}

pub fn fork_proc() {}

pub fn exec() {}
