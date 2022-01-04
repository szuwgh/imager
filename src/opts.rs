use crate::cli::Create;
use crate::container::container::ContainerBuilder;
use crate::utils::ipc::Reader;
use anyhow::{Context, Result};

pub fn create(c: Create) -> Result<()> {
    ContainerBuilder::new(c.container_id, c.bundle);
    Ok(())
}
