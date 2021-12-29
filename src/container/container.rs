use super::state::State;
use crate::oci::oci::Spec;
use crate::utils::fs;
use anyhow::Result;

use std::{collections::HashMap, io::Write, path::PathBuf};
pub struct Container {
    state: State,
    dir: PathBuf,
}

impl Container {
    fn new(id: &str, pid: u64, bundle: PathBuf, dir: PathBuf) -> Self {
        Self {
            state: State::new(id, pid, bundle),
            dir: dir,
        }
    }

    fn save(&self) {
        self.state.save(self.dir);
    }
}

pub struct ContainerBuilder {
    container_id: String,
    bundle: PathBuf,
    root_path: PathBuf,
}

impl ContainerBuilder {
    fn new(container_id: String, bundle: PathBuf) -> Self {
        let root_path = PathBuf::from("/var/run/rsrun");
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
    fn create_container_dir(&self) -> Result<()> {
        let dir = self.root_path.join(&self.container_id);
        fs::create_dir_all(dir)?;
        Ok(())
    }

    fn build(self) -> Result<()> {
        let s = self.load_spec();
        let container_dir = self.create_container_dir()?;
        Ok(())
        // Container {}
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

    fn test_create_container_status() {}
}
