mod common;

use common::{TestHome, names, sample_servers, server};
use oh_my_servers::commands;

// --- add ---

#[test]
fn add_appends_to_actives_yaml() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    commands::add(&mut servers, server("delta", None), "yaml", home.config.clone()).unwrap();

    assert_eq!(names(&servers.actives), ["alpha", "beta", "delta"]);
    assert_eq!(home.read_servers(), servers);
}

#[test]
fn add_appends_to_actives_json() {
    let home = TestHome::with_servers("config.json", &sample_servers());
    let mut servers = sample_servers();

    commands::add(&mut servers, server("delta", None), "json", home.config.clone()).unwrap();

    assert_eq!(home.read_servers(), servers);
}

#[test]
fn add_unsupported_extension_does_not_write() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    let err = commands::add(&mut servers, server("delta", None), "toml", home.config.clone())
        .unwrap_err();

    assert_eq!(err.to_string(), "Unsupported extension");
    assert_eq!(home.read_servers(), sample_servers());
}

#[test]
fn add_fails_on_unwritable_path() {
    let mut servers = sample_servers();
    let path = "/nonexistent/dir/config.yaml".into();
    assert!(commands::add(&mut servers, server("delta", None), "yaml", path).is_err());
}

// --- delete ---

#[test]
fn delete_moves_active_to_inactives() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    commands::delete(&mut servers, false, &"alpha".into(), home.config.clone(), "yaml").unwrap();

    assert_eq!(names(&servers.actives), ["beta"]);
    assert_eq!(names(&servers.inactives), ["gamma", "alpha"]);
    assert_eq!(home.read_servers(), servers);
}

#[test]
fn delete_permanently_removes_active() {
    let home = TestHome::with_servers("config.json", &sample_servers());
    let mut servers = sample_servers();

    commands::delete(&mut servers, true, &"beta".into(), home.config.clone(), "json").unwrap();

    assert_eq!(names(&servers.actives), ["alpha"]);
    assert_eq!(names(&servers.inactives), ["gamma"]);
    assert_eq!(home.read_servers(), servers);
}

#[test]
fn delete_permanently_removes_inactive() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    commands::delete(&mut servers, true, &"gamma".into(), home.config.clone(), "yaml").unwrap();

    assert_eq!(names(&servers.actives), ["alpha", "beta"]);
    assert!(servers.inactives.is_empty());
    assert_eq!(home.read_servers(), servers);
}

#[test]
fn delete_inactive_without_permanently_is_noop() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    commands::delete(&mut servers, false, &"gamma".into(), home.config.clone(), "yaml").unwrap();

    assert_eq!(servers, sample_servers());
}

#[test]
fn delete_unknown_server_is_noop() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    commands::delete(&mut servers, true, &"nope".into(), home.config.clone(), "yaml").unwrap();

    assert_eq!(servers, sample_servers());
    assert_eq!(home.read_servers(), sample_servers());
}

#[test]
fn delete_unsupported_extension() {
    let home = TestHome::with_servers("config.yaml", &sample_servers());
    let mut servers = sample_servers();

    let err = commands::delete(&mut servers, false, &"alpha".into(), home.config.clone(), "toml")
        .unwrap_err();

    assert_eq!(err.to_string(), "Unsupported extension");
    assert_eq!(home.read_servers(), sample_servers());
}

// --- read-only commands (their output is checked in tests/e2e.rs) ---

#[test]
fn read_only_commands_succeed() {
    let servers = sample_servers();
    commands::ls(&servers, false, false).unwrap();
    commands::ls(&servers, true, true).unwrap();
    commands::describe(&servers, &"alpha".into()).unwrap();
    commands::describe(&servers, &"all".into()).unwrap();
    commands::describe(&servers, &"nope".into()).unwrap();
    commands::generate_aliases(&servers).unwrap();
}
