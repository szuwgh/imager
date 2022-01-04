use super::state::State;
use crate::oci::oci::{Namespace, Spec};
use crate::utils::fork::clone_proc;
use crate::utils::fs;
use crate::utils::ipc;
use crate::utils::ipc::Reader;
use anyhow::Result;
use nix::sys::wait::waitpid;
use nix::unistd::execv;
use nix::unistd::execvp;
use nix::unistd::sethostname;
use std::ffi::CString;

use std::{collections::HashMap, io::Write, path::Path, path::PathBuf};
pub struct Container {
    state: State,
    dir: PathBuf,
}

impl Container {
    fn new(id: &str, pid: u64, bundle: PathBuf, dir: &Path) -> Self {
        Self {
            state: State::new(id, pid, bundle),
            dir: dir.to_path_buf(),
        }
    }

    fn save(self) -> Self {
        self.state.save(self.dir.as_path());
        self
    }
}

pub struct ContainerBuilder {
    container_id: String,
    bundle: PathBuf,
    root_path: PathBuf,
}

impl ContainerBuilder {
    pub fn new(container_id: String, bundle: PathBuf) -> Self {
        let root_path = PathBuf::from("/var/run/smog");
        Self {
            container_id: container_id,
            bundle: bundle,
            root_path: root_path,
        }
    }

    fn with_bundle(mut self, bundle: PathBuf) -> Self {
        self.bundle = bundle;
        self
    }

    fn load_spec(&self) -> Result<Spec> {
        let config_path = self.bundle.join("config.json");
        Spec::load(config_path)
    }

    //创建容器文件夹
    fn create_container_dir(&self) -> Result<PathBuf> {
        let dir = self.root_path.join(&self.container_id);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    fn create(self) -> Result<()> {
        let spec = self.load_spec()?;
        let container_dir = self.create_container_dir()?;
        let mut container =
            Container::new(&self.container_id, 0, self.bundle, &container_dir).save();
        let (w_ipc, r_ipc) = ipc::new::<String>()?;
        let namespaces: Vec<Namespace> = match &spec.linux {
            Some(linux) => linux.namespaces.clone().unwrap_or(Vec::new()),
            None => Vec::new(),
        };
        let pid = clone_proc(|| init_process(&r_ipc), &namespaces)?;
        // waitpid(pid, None)?;
        Ok(())
    }

    fn with_root() {}
}

fn init_process(r: &Reader<String>) -> isize {
    println!("inside container");
    sethostname("container").unwrap();
    let mut args: Vec<CString> = Vec::new();
    args.push(CString::new("/bin/bash").unwrap());
    match execv(&CString::new("/bin/bash").unwrap(), &args) {
        Ok(_) => (),
        Err(err) => {
            println!("[ERROR]: {}", err.to_string());
            std::process::exit(1);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_spec() {
        let s = ContainerBuilder::new("".to_owned(), PathBuf::from(env!("CARGO_MANIFEST_DIR")))
            .load_spec()
            .unwrap();
        println!("{:?}", s);
    }
    #[test]
    fn test_create_container_status() {
        ContainerBuilder::new(
            "aabbcc".to_owned(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        )
        .create()
        .unwrap();
    }
}
