use super::state::{State, Status};
use crate::oci::oci::{Namespace, Spec};
use crate::utils::fork::fork_child;
use crate::utils::fs;
use crate::utils::ipc;
use crate::utils::ipc::{NotifyListener, NotifySocket};
use crate::utils::ipc::{Reader, Writer};
use anyhow::{bail, Result};
use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::execv;
use nix::unistd::sethostname;
use std::ffi::CString;
use std::{path::Path, path::PathBuf};

const SOCK_FILE: &'static str = "smog.sock";

pub struct ContainerInstance {
    state: State,
    dir: PathBuf,
}

impl ContainerInstance {
    fn new(state: State, container_dir: &Path) -> Self {
        Self {
            state: state,
            dir: container_dir.to_path_buf(),
        }
    }

    fn load(container_dir: &Path) -> Result<ContainerInstance> {
        let state = State::load(&container_dir)?;
        Ok(Self {
            state: state,
            dir: container_dir.to_path_buf(),
        })
    }

    fn save(&self) -> Result<()> {
        self.state.save(self.dir.as_path())?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        let socket_path = self.dir.join(SOCK_FILE);
        let notify_socket = NotifySocket::new(&socket_path)?;
        notify_socket.notify("start").unwrap();
        notify_socket.close().unwrap();
        self.state.status = Status::Running;
        self.save()?;
        Ok(())
    }
}

pub struct Container {
    container_id: String,
    bundle: PathBuf,
    root_path: PathBuf,
}

impl Container {
    const ROOT_PATH: &'static str = "/var/run/smog";

    pub fn new(container_id: String, bundle: PathBuf) -> Self {
        let root_path = PathBuf::from(Self::ROOT_PATH);
        Self {
            container_id: container_id,
            bundle: bundle,
            root_path: root_path,
        }
    }

    pub fn load(container_id: String) -> Result<ContainerInstance> {
        let root_path = PathBuf::from(Self::ROOT_PATH);
        let container_dir = root_path.join(container_id);
        let container = ContainerInstance::load(&container_dir)?;
        Ok(container)
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

    pub fn create(self) -> Result<()> {
        let spec = self.load_spec()?;
        let container_dir = self.create_container_dir()?;
        let sock_path = container_dir.join(SOCK_FILE);
        let notify_listener = NotifyListener::new(&sock_path)?;
        let (w_ipc, r_ipc) = ipc::new::<String>()?;
        let namespaces: Vec<Namespace> = match &spec.linux {
            Some(linux) => linux.namespaces.clone().unwrap_or(Vec::new()),
            None => Vec::new(),
        };
        let pid = fork_child(|| init_process(&w_ipc, &notify_listener, &namespaces))?;
        let msg = r_ipc.read()?;
        if msg != "ready" {
            bail!("not ready");
        }

        let mut state = State::new(&self.container_id, pid.as_raw(), self.bundle);
        state.status = Status::Created;
        let container = ContainerInstance::new(state, &container_dir);
        container.save()?;
        waitpid(pid, None)?;
        Ok(())
    }

    fn with_root() {}
}

fn init_process(
    w: &Writer<String>,
    notify_listener: &NotifyListener,
    namespaces: &Vec<Namespace>,
) -> Result<()> {
    for v in namespaces.iter() {
        println!("{:?}", v.typ);
    }
    unshare(CloneFlags::CLONE_NEWUTS)?;
    sethostname("container")?;
    w.write("ready".to_owned())?;
    notify_listener.wait_container_start()?;
    notify_listener.close()?;
    do_exec("/bin/sh")?;
    Ok(())
}

fn do_exec(cmd: &str) -> Result<()> {
    let mut args: Vec<CString> = Vec::new();
    args.push(CString::new(cmd).unwrap());
    match execv(&CString::new(cmd).unwrap(), &args) {
        Ok(_) => (),
        Err(err) => {
            // We can't log this error because it doesn't see the log file
            println!("[ERROR]: {}", err.to_string());
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_spec() {
        let s = Container::new("".to_owned(), PathBuf::from(env!("CARGO_MANIFEST_DIR")))
            .load_spec()
            .unwrap();
        println!("{:?}", s);
    }
    #[test]
    fn test_create_container_status() {
        Container::new(
            "aabbcc".to_owned(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        )
        .create()
        .unwrap();
    }
}
