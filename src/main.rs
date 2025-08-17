use std::fmt::Write as _;
use std::result::Result;
use std::{env::home_dir, error::Error, ffi::OsStr, fs::read_dir, path::PathBuf};

mod models;

use models::Servers;

const CONF_DIF: &str = ".oh-my-servers";

fn main() -> Result<(), Box<dyn Error>> {
    let Some(file_path) = find_config_file() else {
        return Err("Configuration file not found".into());
    };

    let extension = file_path
        .extension()
        .and_then(OsStr::to_str)
        .and_then(|ext| match ext {
            "json" => Some("json"),
            "yaml" | "yml" => Some("yaml"),
            _ => None,
        });

    let content = &std::fs::read_to_string(file_path)?;

    let servers = match extension {
        Some("json") => serde_json::from_str::<Servers>(content)?,
        Some("yaml") => serde_yaml::from_str::<Servers>(content)?,
        _ => return Err("Unsupported file format".into()),
    };

    let aliases = generate_aliases(&servers);

    println!("{aliases}");
    Ok(())
}

fn generate_aliases(servers: &Servers) -> String {
    let mut aliases = String::new();
    servers.servers.iter().for_each(|server| {
        if server.key_path.is_some() {
            let s_name = &server.name;
            let s_key = &server.key_path.as_ref().expect("Already checked above");
            let s_user = &server.user;
            let s_ip = &server.ip;
            let _ = writeln!(
                &mut aliases,
                "alias {s_name}='ssh {s_user}@{s_ip} -i {s_key}'"
            );
        } else {
            let s_name = &server.name;
            let s_user = &server.user;
            let s_ip = &server.ip;
            let _ = writeln!(&mut aliases, "alias {s_name}='ssh {s_user}@{s_ip}'");
        }
    });
    aliases
}

/// Find the configuration file path in the `$HOME/.oh-my-servers`.
fn find_config_file() -> Option<PathBuf> {
    let home = home_dir()?;
    let mut files = read_dir(home.join(CONF_DIF)).ok()?;
    let conf_file = files.find(|f| {
        f.is_ok()
            && f.as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("config.")
    })?;
    Some(conf_file.ok()?.path())
}
