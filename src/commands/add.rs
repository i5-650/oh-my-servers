use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::models::{Server, Servers};

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
