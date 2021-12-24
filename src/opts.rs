use crate::cli::Create;
use crate::oci::oci::Spec;
use crate::utils::file;
use anyhow::{Context, Result};
use nix::unistd::Pid;
use std::path::PathBuf;

fn create(c: Create) {}

pub struct Container {
    pid: Pid,
}

pub struct ContainerBuilder {
    container_id: String,
    bundle: PathBuf,
}

impl ContainerBuilder {
    fn new(container_id: String, bundle: PathBuf) -> Self {
        Self {
            container_id: container_id,
            bundle: bundle,
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

    fn build(self) {
        let s = self.load_spec();
        // Container {}
    }

    fn with_root() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_spec() {
        let s = ContainerBuilder::new("".to_owned(), PathBuf::from("/opt/rsproject/rsrun"))
            .load_spec()
            .unwrap();
        println!("{:?}", s);
    }
}
