use crate::common::error::Error;
use crate::oci::oci::Namespace;
use anyhow::Result;
use libc::c_int;
use nix::sched::{clone, CloneFlags};
use nix::sys::signal::Signal;
use nix::unistd::{fork, ForkResult, Pid};

fn to_clone_flags(namespace: &Namespace) -> CloneFlags {
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

pub fn fork_child<F: FnOnce() -> Result<()>>(f: F) -> Result<Pid> {
    match unsafe { fork()? } {
        ForkResult::Parent { child } => Ok(child),
        ForkResult::Child => {
            let ret = if let Err(error) = f() {
                println!("error:{}", error);
                -1
            } else {
                0
            };
            std::process::exit(ret);
        }
    }
}
