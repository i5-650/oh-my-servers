//! Runs the real binary against a temporary `$HOME`.

mod common;

use assert_cmd::Command;
use common::{TestHome, names, sample_servers};
use predicates::prelude::*;
use predicates::str::contains;
use tempfile::TempDir;

fn cmd(home: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("oh-my-servers").unwrap();
    cmd.env("HOME", home)
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("SHELL")
        .env_remove("OMS_CONFIG");
    cmd
}

fn yaml_home() -> TestHome {
    TestHome::with_servers("config.yaml", &sample_servers())
}

// --- configuration errors ---

#[test]
fn missing_config_dir() {
    let home = TempDir::new().unwrap();
    cmd(home.path())
        .arg("ls")
        .assert()
        .failure()
        .stderr(contains("Configuration file not found"));
}

#[test]
fn unsupported_config_format() {
    let home = TestHome::with_raw_config("config.toml", "");
    cmd(home.path())
        .arg("ls")
        .assert()
        .failure()
        .stderr(contains("Unsupported file format"));
}

#[test]
fn invalid_config_content() {
    let home = TestHome::with_raw_config("config.yaml", "actives: 42");
    cmd(home.path()).arg("ls").assert().failure();
}

#[test]
fn yml_extension_is_supported() {
    let home = TestHome::with_servers("config.yml", &sample_servers());
    cmd(home.path()).arg("ls").assert().success().stdout(contains("alpha"));
}

#[test]
fn json_config_is_supported() {
    let home = TestHome::with_servers("config.json", &sample_servers());
    cmd(home.path()).arg("ls").assert().success().stdout(contains("alpha"));
}

#[test]
fn oms_config_overrides_home() {
    let home = TestHome::with_servers("servers.json", &sample_servers());
    let empty_home = TempDir::new().unwrap();
    cmd(empty_home.path())
        .env("OMS_CONFIG", &home.config)
        .arg("ls")
        .assert()
        .success()
        .stdout(contains("alpha"));
}

#[test]
fn oms_config_missing_file_fails() {
    let home = yaml_home();
    cmd(home.path())
        .env("OMS_CONFIG", home.path().join("nope.yaml"))
        .arg("ls")
        .assert()
        .failure();
}

#[test]
fn empty_oms_config_falls_back_to_home() {
    let home = yaml_home();
    cmd(home.path())
        .env("OMS_CONFIG", "")
        .arg("ls")
        .assert()
        .success()
        .stdout(contains("alpha"));
}

#[test]
fn invalid_arguments_fail() {
    let home = yaml_home();
    cmd(home.path()).arg("nope").assert().failure().code(2);
}

// --- ls ---

#[test]
fn ls_lists_actives_only() {
    let home = yaml_home();
    cmd(home.path())
        .arg("ls")
        .assert()
        .success()
        .stdout("alpha:\t alpha server\nbeta:\t beta server\n");
}

#[test]
fn ls_all_lists_inactives() {
    let home = yaml_home();
    cmd(home.path())
        .args(["ls", "--all"])
        .assert()
        .success()
        .stdout(
            "actives\nalpha:\t alpha server\nbeta:\t beta server\n\
             inactives\ngamma:\t gamma server\n",
        );
}

#[test]
fn ls_long_format() {
    let home = yaml_home();
    cmd(home.path())
        .args(["ls", "-l"])
        .assert()
        .success()
        .stdout(contains("alpha\n\tHost: alpha.example.com\n\tPort: 22"))
        .stdout(contains("Authentication: ssh key ~/.ssh/alpha"))
        .stdout(contains("Authentication: password"))
        .stdout(contains("gamma").not());
}

// --- describe ---

#[test]
fn describe_server() {
    let home = yaml_home();
    cmd(home.path())
        .args(["describe", "beta"])
        .assert()
        .success()
        .stdout(
            "beta\n\tHost: beta.example.com\n\tPort: 22\n\tUser: admin\n\tOS: debian\n\
             \tDescription: beta server\n\tAuthentication: password\n",
        );
}

#[test]
fn describe_all() {
    let home = yaml_home();
    cmd(home.path())
        .args(["describe", "all"])
        .assert()
        .success()
        .stdout(contains("Host: alpha.example.com"))
        .stdout(contains("Host: beta.example.com"))
        .stdout(contains("gamma").not());
}

#[test]
fn describe_unknown_server() {
    let home = yaml_home();
    cmd(home.path())
        .args(["describe", "gamma"])
        .assert()
        .success()
        .stdout("No such resources\n");
}

// --- shell ---

#[test]
fn shell_generates_aliases() {
    let home = yaml_home();
    cmd(home.path())
        .arg("shell")
        .assert()
        .success()
        .stdout(
            "alias alpha='ssh admin@alpha.example.com -i ~/.ssh/alpha'\n\
             alias beta='ssh admin@beta.example.com'\n\n",
        );
}

// --- completion ---

#[test]
fn completion_for_given_shell() {
    let home = yaml_home();
    cmd(home.path())
        .args(["completion", "zsh"])
        .assert()
        .success()
        .stdout(contains("#compdef oh-my-servers"));
}

#[test]
fn completion_defaults_to_bash() {
    let home = yaml_home();
    cmd(home.path())
        .arg("completion")
        .assert()
        .success()
        .stdout(contains("_oh-my-servers()"));
}

#[test]
fn completion_uses_shell_env() {
    let home = yaml_home();
    cmd(home.path())
        .arg("completion")
        .env("SHELL", "/usr/bin/fish")
        .assert()
        .success()
        .stdout(contains("complete -c oh-my-servers"));
}

// --- add / delete ---

#[test]
fn add_persists_server() {
    let home = yaml_home();
    cmd(home.path())
        .args([
            "add", "-n", "delta", "-H", "10.0.0.4", "-p", "2222", "-u", "root", "-o", "arch", "-k",
            "~/.ssh/delta",
        ])
        .assert()
        .success();

    let servers = home.read_servers();
    assert_eq!(names(&servers.actives), ["alpha", "beta", "delta"]);
    let delta = &servers.actives[2];
    assert_eq!(delta.host, "10.0.0.4");
    assert_eq!(delta.port, Some(2222));
    assert_eq!(delta.description, "No description");
    assert_eq!(delta.key_path.as_deref(), Some("~/.ssh/delta"));

    cmd(home.path())
        .arg("shell")
        .assert()
        .success()
        .stdout(contains("alias delta='ssh root@10.0.0.4 -i ~/.ssh/delta'"));
}

#[test]
fn add_persists_json() {
    let home = TestHome::with_servers("config.json", &sample_servers());
    cmd(home.path())
        .args(["add", "-n", "delta", "-H", "h", "-u", "u", "-o", "o"])
        .assert()
        .success();

    assert_eq!(
        names(&home.read_servers().actives),
        ["alpha", "beta", "delta"]
    );
}

#[test]
fn delete_soft_then_permanently() {
    let home = yaml_home();

    cmd(home.path()).args(["delete", "alpha"]).assert().success();
    let servers = home.read_servers();
    assert_eq!(names(&servers.actives), ["beta"]);
    assert_eq!(names(&servers.inactives), ["gamma", "alpha"]);

    cmd(home.path())
        .args(["delete", "alpha", "--permanently"])
        .assert()
        .success();
    let servers = home.read_servers();
    assert_eq!(names(&servers.actives), ["beta"]);
    assert_eq!(names(&servers.inactives), ["gamma"]);
}
