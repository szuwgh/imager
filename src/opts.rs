use crate::cli::{Create, Run, Start};
use crate::container::container::Container;
use crate::utils::ipc::Reader;
use anyhow::{Context, Result};
use nix::sys::wait::waitpid;

pub fn create(c: Create) -> Result<()> {
    Container::new(c.container_id, c.bundle).create()?;
    Ok(())
}

pub fn start(c: Start) -> Result<()> {
    let mut container = Container::load(c.container_id)?;
    container.start()?;
    Ok(())
}

pub fn run(r: Run) -> Result<()> {
    let (mut container, pid) = Container::new(r.container_id, r.bundle).create()?;
    container.start()?;
    waitpid(pid, None)?;
    Ok(())
}
