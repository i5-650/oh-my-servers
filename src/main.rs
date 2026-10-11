use anyhow::{Error, anyhow};
use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};

use oh_my_servers::{
    cli::{Args, Commands},
    commands,
    config::{find_config_file, get_extension, parse_config},
    models::Server,
};

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
        Commands::Ls { all, long } => commands::ls(&servers, *all, *long),
        Commands::Describe { server } => commands::describe(&servers, server),
        Commands::Completion { shell } => {
            let selected_shell = shell
                .to_owned()
                .or_else(Shell::from_env)
                .unwrap_or(Shell::Bash);

            let mut cmd = Args::command();
            generate(
                selected_shell,
                &mut cmd,
                "oh-my-servers",
                &mut std::io::stdout(),
            );
            Ok(())
        }
        Commands::Delete {
            server,
            permanently,
        } => commands::delete(&mut servers, *permanently, server, file_path, extension),
        Commands::Add {
            name,
            host,
            port,
            user,
            os,
            description,
            key_path,
        } => commands::add(
            &mut servers,
            Server::new(
                name,
                host,
                *port,
                user,
                os,
                description.to_owned(),
                key_path.to_owned(),
            ),
            extension,
            file_path,
        ),
    }
}
