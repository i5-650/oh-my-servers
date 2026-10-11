mod common;

use std::path::{Path, PathBuf};

use common::{TestHome, sample_servers};
use oh_my_servers::config::{CONF_DIR, find_config_file_in, get_extension, parse_config};
use tempfile::TempDir;

#[test]
fn get_extension_supported_formats() {
    assert_eq!(get_extension(Path::new("config.json")), Some("json"));
    assert_eq!(get_extension(Path::new("config.yaml")), Some("yaml"));
    assert_eq!(get_extension(Path::new("config.yml")), Some("yaml"));
}

#[test]
fn get_extension_unsupported_formats() {
    assert_eq!(get_extension(Path::new("config.txt")), None);
    assert_eq!(get_extension(Path::new("config.JSON")), None);
    assert_eq!(get_extension(Path::new("config")), None);
}

#[test]
fn find_config_file_yaml() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    assert_eq!(find_config_file_in(home.path()), Some(home.config.clone()));
}

#[test]
fn find_config_file_ignores_other_files() {
    let home = TestHome::with_servers("config.json", &sample_servers());
    std::fs::write(home.path().join(CONF_DIR).join("notes.txt"), "").unwrap();
    assert_eq!(find_config_file_in(home.path()), Some(home.config.clone()));
}

#[test]
fn find_config_file_missing_directory() {
    let home = TempDir::new().unwrap();
    assert_eq!(find_config_file_in(home.path()), None);
}

#[test]
fn find_config_file_no_config_in_directory() {
    let home = TestHome::with_raw_config("other.yaml", "");
    assert_eq!(find_config_file_in(home.path()), None);
}

#[test]
fn parse_config_yaml() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    assert_eq!(
        parse_config(&home.config, "yaml").unwrap(),
        sample_servers()
    );
}

#[test]
fn parse_config_json() {
    let home = TestHome::with_servers("config.json", &sample_servers());
    assert_eq!(
        parse_config(&home.config, "json").unwrap(),
        sample_servers()
    );
}

#[test]
fn parse_config_unsupported_extension() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let err = parse_config(&home.config, "toml").unwrap_err();
    assert_eq!(err.to_string(), "Unsupported file format");
}

#[test]
fn parse_config_invalid_content() {
    let home = TestHome::with_raw_config("config.json", "{ not json");
    assert!(parse_config(&home.config, "json").is_err());
}

#[test]
fn parse_config_missing_file() {
    assert!(parse_config(&PathBuf::from("/nonexistent/config.yaml"), "yaml").is_err());
}
