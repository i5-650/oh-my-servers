use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::models::Servers;

/// Move `server` to the inactive servers, or remove it entirely when
/// `permanently` is set, then save the configuration to `file_path`.
///
/// # Errors
///
/// Returns an error if `extension` is not supported, or if the configuration
/// cannot be serialized or written.
pub fn delete(
    servers: &mut Servers,
    permanently: bool,
    server: &String,
    file_path: PathBuf,
    extension: &str,
) -> Result<()> {
    if let Some(idx) = servers.actives.iter().position(|s| s.name == *server) {
        let removed = servers.actives.remove(idx);
        if !permanently {
            servers.inactives.push(removed);
        }
    } else if permanently
        && let Some(idx) = servers.inactives.iter().position(|s| s.name == *server)
    {
        servers.inactives.remove(idx);
    }

    let content = match extension {
        "json" => serde_json::to_string(servers)?,
        "yaml" => serde_yaml::to_string(servers)?,
        _ => {
            return Err(anyhow!("Unsupported extension"));
        }
    };

    Ok(std::fs::write(file_path, content)?)
}
