use crate::cli::Create;
use crate::oci::Spec;
use crate::utils::file;
use anyhow::{Context, Result};
use std::path::PathBuf;

fn create(c: Create) {}

pub struct Container {}

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

    fn load_spec(&self) -> Result<()> {
        let config_path = self.bundle.join("config.json");
        //let config = std::fs::read_to_string("cluster.json")?;
        Ok(())
    }

    fn build(self) -> Container {
        Container {}
    }

    fn with_root() {}
}
