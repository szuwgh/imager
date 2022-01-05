use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::PathBuf;
use std::{collections::HashMap, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
    Paused,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub oci_version: String,
    pub id: String,
    pub status: Status,
    pub pid: i32,
    pub bundle: PathBuf,
    pub annotations: Option<HashMap<String, String>>,
}

impl State {
    const OCI_VERSION: &'static str = "1.0.2";
    const STATE_FILE: &'static str = "state.json";
    pub fn new(id: &str, pid: i32, bundle: PathBuf) -> Self {
        Self {
            oci_version: Self::OCI_VERSION.to_string(),
            id: id.to_string(),
            status: Status::Creating,
            pid: pid,
            bundle: bundle,
            annotations: Some(HashMap::default()),
        }
    }

    pub fn load(container_dir: &Path) -> Result<State> {
        let file_path = Self::state_file_path(container_dir);
        let file = File::open(&file_path)?;
        let state = serde_json::from_reader(&file)?;
        Ok(state)
    }

    fn state_file_path(container_dir: &Path) -> PathBuf {
        container_dir.join(Self::STATE_FILE)
    }

    pub fn save(&self, container_dir: &Path) -> Result<()> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .create(true)
            .truncate(true)
            .open(Self::state_file_path(container_dir))?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}
