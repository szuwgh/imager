use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub oci_version: String,
    pub root: Option<Root>,
}

impl Spec {
    fn load(config: PathBuf) -> Result<Spec> {
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
