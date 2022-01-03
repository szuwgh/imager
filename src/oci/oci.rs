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

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub path: String,
    pub readonly: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    pub namespaces: Option<Vec<Namespace>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub typ: String,
    pub path: Option<String>,
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
