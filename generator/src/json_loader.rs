use anyhow::{ensure, Result};
use rustdoc_types::{Crate, FORMAT_VERSION};
use serde::Deserialize;

#[derive(Deserialize)]
struct CrateFormatVersion {
    format_version: u32,
}

pub fn load_rjd(input: &str) -> Result<Crate> {
    let format: CrateFormatVersion = serde_json::from_str(input)?;
    ensure!(
        format.format_version == FORMAT_VERSION,
        "Unsupported format version: {}",
        format.format_version
    );
    let krate: Crate = serde_json::from_str(input)?;
    Ok(krate)
}
