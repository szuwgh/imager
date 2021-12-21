use clap::Parser;
use std::path::PathBuf;

/// Create a container
#[derive(Parser, Debug)]
pub struct Create {
    #[clap(short, long, default_value = ".")]
    pub bundle: PathBuf,
    #[clap(forbid_empty_values = true, required = true)]
    pub container_id: String,
}