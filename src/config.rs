use anyhow::{Result, anyhow};
use std::{
    env::{self, home_dir},
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::models::Servers;

pub const CONF_DIR: &str = ".oh-my-servers";
/// Environment variable overriding the configuration file path.
pub const CONFIG_ENV: &str = "OMS_CONFIG";

pub fn get_extension(file_path: &Path) -> Option<&str> {
    file_path
        .extension()
        .and_then(OsStr::to_str)
        .and_then(|ext| match ext {
            "json" => Some("json"),
            "yaml" | "yml" => Some("yaml"),
            _ => None,
        })
}

/// Parse the configuration file at `file_path` as `extension` (`json` or `yaml`).
///
/// # Errors
///
/// Returns an error if the file cannot be read, its content is invalid, or
/// `extension` is not supported.
pub fn parse_config(file_path: &PathBuf, extension: &str) -> Result<Servers> {
    let content = &std::fs::read_to_string(file_path)?;

    match extension {
        "json" => Ok(serde_json::from_str::<Servers>(content)?),
        "yaml" => Ok(serde_yaml::from_str::<Servers>(content)?),
        _ => Err(anyhow!("Unsupported file format")),
    }
}

/// Find the configuration file path: `$OMS_CONFIG` if set and non-empty,
/// otherwise the `config.*` file in `$HOME/.oh-my-servers`.
#[must_use]
pub fn find_config_file() -> Option<PathBuf> {
    match env::var_os(CONFIG_ENV) {
        Some(path) if !path.is_empty() => Some(PathBuf::from(path)),
        _ => find_config_file_in(&home_dir()?),
    }
}

/// Find the configuration file path in `<home>/.oh-my-servers`.
#[must_use]
pub fn find_config_file_in(home: &Path) -> Option<PathBuf> {
    read_dir(home.join(CONF_DIR))
        .ok()?
        .filter_map(Result::ok)
        .find(|f| f.file_name().to_string_lossy().starts_with("config."))
        .map(|f| f.path())
}
