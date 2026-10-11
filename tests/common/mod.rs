#![allow(dead_code)]

use std::path::{Path, PathBuf};

use oh_my_servers::{
    config::CONF_DIR,
    models::{Server, Servers},
};
use tempfile::TempDir;

pub fn server(name: &str, key_path: Option<&str>) -> Server {
    Server::new(
        &name.to_string(),
        &format!("{name}.example.com"),
        None,
        &"admin".to_string(),
        &"debian".to_string(),
        Some(format!("{name} server")),
        key_path.map(str::to_string),
    )
}

/// Two actives (`alpha` with a key, `beta` without) and one inactive (`gamma`).
pub fn sample_servers() -> Servers {
    Servers {
        actives: vec![server("alpha", Some("~/.ssh/alpha")), server("beta", None)],
        inactives: vec![server("gamma", None)],
    }
}

pub fn names(servers: &[Server]) -> Vec<&str> {
    servers.iter().map(|s| s.name.as_str()).collect()
}

/// A fake `$HOME` containing `.oh-my-servers/<file_name>`.
pub struct TestHome {
    pub dir: TempDir,
    pub config: PathBuf,
}

impl TestHome {
    pub fn with_raw_config(file_name: &str, content: &str) -> Self {
        let dir = TempDir::new().unwrap();
        let conf_dir = dir.path().join(CONF_DIR);
        std::fs::create_dir(&conf_dir).unwrap();
        let config = conf_dir.join(file_name);
        std::fs::write(&config, content).unwrap();
        Self { dir, config }
    }

    pub fn with_servers(file_name: &str, servers: &Servers) -> Self {
        let content = if Path::new(file_name).extension().is_some_and(|ext| ext == "json") {
            serde_json::to_string(servers).unwrap()
        } else {
            serde_yaml::to_string(servers).unwrap()
        };
        Self::with_raw_config(file_name, &content)
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn read_servers(&self) -> Servers {
        let content = std::fs::read_to_string(&self.config).unwrap();
        if self.config.extension().unwrap() == "json" {
            serde_json::from_str(&content).unwrap()
        } else {
            serde_yaml::from_str(&content).unwrap()
        }
    }
}
