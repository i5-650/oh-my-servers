use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::models::Servers;

pub fn rm(
    servers: &mut Servers,
    server: &String,
    file_path: PathBuf,
    extension: &str,
) -> Result<()> {
    let to_remove = servers.servers.iter().find(|s| s.name == *server);
    let idx = servers.servers.iter().position(|s| s.name == *server);

    if let (Some(to_remove), Some(idx)) = (to_remove, idx) {
        servers.inactives.push(to_remove.to_owned());
        servers.servers.remove(idx);
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
