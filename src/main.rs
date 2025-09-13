use anyhow::{Error, Result, anyhow};
use clap::{Parser, Subcommand};
use std::{
    env::home_dir,
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

mod commands;
mod models;

use models::Servers;

const CONF_DIF: &str = ".oh-my-servers";

#[derive(Parser)]
#[command(
    author = "i5-650",
    about = "Oh My Servers - Server management tool",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate shell integration (aliases, completions, etc.)
    Shell {
        // #[arg(
        //     value_name = "shell",
        //     required = false,
        //     help = "The shell to generate integration for (bash, zsh, fish, etc.)"
        // )]
        // shell: Option<String>,
    },

    /// List servers
    Ls,

    /// Describe a server
    Describe {
        #[arg(value_name = "server", help = "The name of the server to describe")]
        server: String,
    },

    /// Add a new server
    Add {
        #[arg(value_name = "todo", help = "todo")]
        connection: String,
    },

    /// Remove a server
    Delete {
        #[arg(value_name = "server", help = "The name of the server to remove")]
        server: String,

        #[arg(
            value_name = "permanently",
            required = false,
            short = 'p',
            long = "permanently",
            help = "Whether to delete permanently the server"
        )]
        permanently: bool,
    },
}

fn main() -> Result<(), Error> {
    let Some(file_path) = find_config_file() else {
        return Err(anyhow!("Configuration file not found"));
    };

    let extension = get_extension(&file_path);

    let extension = match extension {
        Some("json") => "json",
        Some("yaml") => "yaml",
        _ => return Err(anyhow!("Unsupported file format")),
    };

    let mut servers = parse_config(&file_path, extension)?;

    let args = Args::parse();

    match &args.command {
        Commands::Shell {} => commands::generate_aliases(&servers),
        Commands::Ls => commands::ls(&servers),
        Commands::Describe { server } => commands::describe(&servers, server),
        Commands::Delete {
            server,
            permanently,
        } => {
            commands::delete(&mut servers, *permanently, server, file_path, extension)?;
        }
        Commands::Add { connection } => {
            println!("Noop: {connection}");
        }
    }

    Ok(())
}

fn get_extension(file_path: &Path) -> Option<&str> {
    file_path
        .extension()
        .and_then(OsStr::to_str)
        .and_then(|ext| match ext {
            "json" => Some("json"),
            "yaml" | "yml" => Some("yaml"),
            _ => None,
        })
}

fn parse_config(file_path: &PathBuf, extension: &str) -> Result<Servers> {
    let content = &std::fs::read_to_string(file_path)?;

    match extension {
        "json" => Ok(serde_json::from_str::<Servers>(content)?),
        "yaml" => Ok(serde_yaml::from_str::<Servers>(content)?),
        _ => Err(anyhow!("Unsupported file format")),
    }
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
