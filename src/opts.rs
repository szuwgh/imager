use crate::cli::{Create, Start};
use crate::container::container::Container;
use crate::utils::ipc::Reader;
use anyhow::{Context, Result};

pub fn create(c: Create) -> Result<()> {
    Container::new(c.container_id, c.bundle).create()?;
    Ok(())
}

pub fn start(c: Start) -> Result<()> {
    let mut container = Container::load(c.container_id)?;
    container.start()?;
    Ok(())
}
