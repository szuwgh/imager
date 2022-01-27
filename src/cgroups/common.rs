use crate::oci::oci::LinuxResources;
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct ControllerOpt<'a> {
    pub resources: &'a LinuxResources,
}

#[inline]
pub fn write_cgroup_file_str<P: AsRef<Path>>(path: P, data: &str) -> Result<()> {
    fs::OpenOptions::new()
        .create(false)
        .write(true)
        .truncate(false)
        .open(path.as_ref())?
        .write_all(data.as_bytes())?;
    Ok(())
}

#[inline]
pub fn write_cgroup_file<P: AsRef<Path>, T: ToString>(path: P, data: T) -> Result<()> {
    fs::OpenOptions::new()
        .create(false)
        .write(true)
        .truncate(false)
        .open(path.as_ref())?
        .write_all(data.to_string().as_bytes())?;

    Ok(())
}
