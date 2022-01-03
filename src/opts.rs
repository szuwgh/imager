use crate::cli::Create;
use crate::oci::oci::Spec;
use crate::utils::fork::clone_proc;
use anyhow::{Context, Result};
use nix::sys::wait::waitpid;
use nix::unistd::execv;
use nix::unistd::execvp;
use nix::unistd::sethostname;
use nix::unistd::Pid;
use std::ffi::CString;
use std::path::Path;
use std::path::PathBuf;
use std::thread;

pub fn create(c: Create) -> Result<()> {
    let spec = Spec::load(Path::new("/opt/rsproject/smog/config.json").to_path_buf())?;
    let namespaces = match &spec.linux {
        Some(linux) => linux.namespaces.clone().unwrap_or(Vec::new()),
        None => Vec::new(),
    };
    println!("{:?}", namespaces);
    let pid = clone_proc(|| init_process(), &namespaces)?;
    // waitpid(pid, None)?;
    println!("exit parent");
    Ok(())
}

fn init_process() -> isize {
    println!("inside container");
    sethostname("container").unwrap();
    let mut args: Vec<CString> = Vec::new();
    args.push(CString::new("/bin/sh").unwrap());
    match execv(&CString::new("/bin/sh").unwrap(), &args) {
        Ok(_) => (),
        Err(err) => {
            // We can't log this error because it doesn't see the log file
            println!("[ERROR]: {}", err.to_string());
            std::process::exit(1);
        }
    }
    //  thread::sleep_ms(10000);
    0
}
