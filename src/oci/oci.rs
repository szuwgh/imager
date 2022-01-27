use super::oci_error;
use super::OciError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub oci_version: String,
    pub root: Option<Root>,
    pub linux: Option<Linux>,
}

impl Spec {
    pub fn load(config: PathBuf) -> Result<Spec> {
        let file = File::open(config)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub path: PathBuf,
    pub readonly: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    pub namespaces: Option<Vec<Namespace>>,
    pub resources: Option<LinuxResources>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResources {
    pub cpu: Option<LinuxCpu>,
    pub memory: Option<LinuxMemory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxCpu {
    pub shares: Option<u64>,
    pub quota: Option<i64>,
    pub period: Option<u64>,
    pub realtime_runtime: Option<i64>,
    pub realtime_period: Option<u64>,
    pub cpus: Option<String>,
    pub mems: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxMemory {
    limit: Option<i64>,
    reservation: Option<i64>,
    swap: Option<i64>,
    kernel: Option<i64>,
    kernel_tcp: Option<i64>,
    swappiness: Option<u64>,
    disable_oom_killer: Option<bool>,
    use_hierarchy: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceType {
    Mount,
    Cgroup,
    Uts,
    Ipc,
    User,
    Pid,
    Network,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub typ: NamespaceType,
    pub path: Option<String>,
}

impl TryFrom<&str> for NamespaceType {
    type Error = OciError;
    fn try_from(namespace: &str) -> std::result::Result<Self, Self::Error> {
        match namespace {
            "pid" => Ok(NamespaceType::Pid),
            "mount" => Ok(NamespaceType::Mount),
            "cgroup" => Ok(NamespaceType::Cgroup),
            "uts" => Ok(NamespaceType::Uts),
            "ipc" => Ok(NamespaceType::Ipc),
            "user" => Ok(NamespaceType::User),
            "network" => Ok(NamespaceType::Network),
            _ => Err(oci_error(format!("unknown namespace {}", namespace))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serde() {
        let s = r#"{"oci_version":"1"}"#;
        let v: Spec = serde_json::from_str(&s).unwrap();
        println!("{:?}", v);
    }
}
