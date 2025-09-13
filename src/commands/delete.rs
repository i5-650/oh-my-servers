use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::models::Servers;

pub fn delete(
    servers: &mut Servers,
    permanently: bool,
    server: &String,
    file_path: PathBuf,
    extension: &str,
) -> Result<()> {
    let mut to_remove = servers.actives.iter().find(|s| s.name == *server);
    if permanently && to_remove.is_none() {
        to_remove = servers.inactives.iter().find(|s| s.name == *server);
    }

    let mut idx = servers.actives.iter().position(|s| s.name == *server);
    if permanently && idx.is_none() {
        idx = servers.inactives.iter().position(|s| s.name == *server);
    }

    if let (Some(to_remove), Some(idx)) = (to_remove, idx) {
        if !permanently {
            servers.inactives.push(to_remove.to_owned());
        }
        servers.actives.remove(idx);
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
