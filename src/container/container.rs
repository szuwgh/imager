use super::state::State;
use crate::oci::oci::Spec;
use crate::utils::fs;
use anyhow::Result;

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
    fn new(container_id: String, bundle: PathBuf) -> Self {
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
        let spec = self.load_spec();
        let container_dir = self.create_container_dir()?;
        let mut container =
            Container::new(&self.container_id, 0, self.bundle, &container_dir).save();

        Ok(())
    }

    fn with_root() {}
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
