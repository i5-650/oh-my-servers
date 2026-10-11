use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::models::{Server, Servers};

/// Add `new_server` to the active servers and save the configuration to `file_path`.
///
/// # Errors
///
/// Returns an error if `extension` is not supported, or if the configuration
/// cannot be serialized or written.
pub fn add(
    servers: &mut Servers,
    new_server: Server,
    extension: &str,
    file_path: PathBuf,
) -> Result<()> {
    servers.actives.push(new_server);

    let content = match extension {
        "json" => serde_json::to_string(servers)?,
        "yaml" => serde_yaml::to_string(servers)?,
        _ => {
            return Err(anyhow!("Unsupported extension"));
        }
    };

    Ok(std::fs::write(file_path, content)?)
}
