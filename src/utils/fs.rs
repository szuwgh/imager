use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
pub fn get_exe_path() -> Result<PathBuf, io::Error> {
    std::env::current_exe()
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    fs::create_dir_all(path).with_context(|| format!("failed to create directory {:?}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_exe_path() {
        let path = get_exe_path();
        println!("{:?}", path);
    }
}
