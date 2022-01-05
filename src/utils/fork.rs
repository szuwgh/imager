use crate::common::error::Error;
use crate::oci::oci::Namespace;
use anyhow::Result;
use libc::c_int;
use nix::sched::{clone, CloneFlags};
use nix::sys::signal::Signal;
use nix::unistd::{fork, ForkResult, Pid};

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
