use super::state::{State, Status};
use crate::cgroups::common::ControllerOpt;
use crate::cgroups::CgroupManager;
use crate::cgroups::CgroupVersion;
use crate::cgroups::{v1, v2};
use crate::oci::oci::{Namespace, NamespaceType, Spec};
use crate::utils::fork::fork_child;
use crate::utils::fs;
use crate::utils::ipc;
use crate::utils::ipc::{NotifyListener, NotifySocket};
use crate::utils::ipc::{Reader, Writer};
use anyhow::{bail, Context, Result};
use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::sys::statfs::statfs;
use nix::sys::statfs::{CGROUP2_SUPER_MAGIC, TMPFS_MAGIC};
use nix::unistd::Pid;

use nix::unistd::{chdir, execv, pivot_root, sethostname};
use std::ffi::CString;
use std::{path::Path, path::PathBuf};
const SOCK_FILE: &'static str = "smog.sock";
pub const DEFAULT_CGROUP_ROOT: &str = "/sys/fs/cgroup";

pub struct ContainerInstance {
    pub state: State,
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

    fn set_bundle(mut self, bundle: PathBuf) -> Self {
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

    pub fn create(self) -> Result<(ContainerInstance, Pid)> {
        let spec = self.load_spec()?;
        //spec.linux.and_then()
        let linux = spec.linux.as_ref().context("no linux in spec")?;

        let container_dir = self.create_container_dir()?;
        let sock_path = container_dir.join(SOCK_FILE);
        let notify_listener = NotifyListener::new(&sock_path)?;
        let (w_ipc, r_ipc) = ipc::new::<String>()?;
        let namespaces: Vec<Namespace> = match &spec.linux {
            Some(linux) => linux.namespaces.clone().unwrap_or(Vec::new()),
            None => Vec::new(),
        };
        let manager = new_cgroup_manager(&self.container_id)?;

        let pid = fork_child(|| init_process(&w_ipc, &spec, &notify_listener, &namespaces))?;
        manager.add_task(pid)?;
        if let Some(r) = linux.resources.as_ref() {
            manager.apply(&ControllerOpt { resources: r })?;
        }
        let msg = r_ipc.read()?;
        if msg != "ready" {
            bail!("not ready");
        }
        let mut state = State::new(&self.container_id, pid.as_raw(), self.bundle);
        state.status = Status::Created;
        let container = ContainerInstance::new(state, &container_dir);
        container.save()?;
        Ok((container, pid))
    }

    pub fn run(self) -> Result<()> {
        Ok(())
    }
}

fn get_cgroup_version() -> Result<CgroupVersion> {
    let default_root = Path::new(DEFAULT_CGROUP_ROOT);
    return match default_root.exists() {
        true => {
            let stat = statfs(default_root)?;
            match stat.filesystem_type() {
                CGROUP2_SUPER_MAGIC => Ok(CgroupVersion::V2),
                TMPFS_MAGIC => Ok(CgroupVersion::V2),
                _ => bail!("non default cgroup root not supported"),
            }
        }
        false => {
            bail!("non default cgroup root not supported")
        }
    };
}

fn new_cgroup_manager(container_id: &str) -> Result<Box<dyn CgroupManager>> {
    let m: Box<dyn CgroupManager> = match get_cgroup_version()? {
        CgroupVersion::V1 => Box::new(v1::manager::Manager::new(container_id)),
        CgroupVersion::V2 => Box::new(v2::manager::Manager::new(
            DEFAULT_CGROUP_ROOT.into(),
            container_id,
        )),
    };
    Ok(m)
}

fn init_process(
    w: &Writer<String>,
    spec: &Spec,
    notify_listener: &NotifyListener,
    namespaces: &Vec<Namespace>,
) -> Result<()> {
    for v in namespaces.iter() {
        match v.typ {
            NamespaceType::Uts => {
                unshare(CloneFlags::CLONE_NEWUTS)?;
                sethostname("container")?;
            }
            NamespaceType::Ipc => {
                unshare(CloneFlags::CLONE_NEWIPC)?;
            }
            NamespaceType::User => {
                unshare(CloneFlags::CLONE_NEWUSER)?;
            }
            NamespaceType::Mount => {
                unshare(CloneFlags::CLONE_NEWNS)?;
                let ref rootfs = spec.root.as_ref().unwrap().path;
                prepare_roofs(rootfs)?;
                pivot_rootfs(rootfs)?;
            }
            NamespaceType::Network => {
                unshare(CloneFlags::CLONE_NEWNET)?;
            }
            _ => {}
        }
    }
    w.write("ready".to_owned())?;
    notify_listener.wait_container_start()?;
    do_exec("/bin/sh")?;
    Ok(())
}

//准备文件系统
fn prepare_roofs(rootfs: &Path) -> Result<()> {
    //https://man7.org/linux/man-pages/man2/pivot_root.2.html
    mount::<str, str, str, str>(None, "/", None, MsFlags::MS_PRIVATE | MsFlags::MS_REC, None)?;
    //  we need this to satisfy restriction:
    // "new_root and put_old must not be on the same filesystem as the current root"
    mount::<Path, Path, str, str>(
        Some(rootfs),
        rootfs,
        None,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None,
    )?;
    Ok(())
}

//povit_root的新目录不能和原来的root目录在一个文件系统上
fn pivot_rootfs(rootfs: &Path) -> Result<()> {
    chdir(rootfs)?;
    let old_root = rootfs.join("oldroot");
    fs::create_dir_all(&old_root)?;
    pivot_root(rootfs.as_os_str(), old_root.as_os_str())?;
    umount2("./oldroot", MntFlags::MNT_DETACH)?;
    fs::remove_dir_all("./oldroot")?;
    chdir("/")?;
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
