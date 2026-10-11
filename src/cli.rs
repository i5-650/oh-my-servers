use clap::{Parser, Subcommand};
use clap_complete::Shell;

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

    Completion {
        #[arg(
            value_enum,
            help = "Shell to generate completions for (bash, zsh, fish, powershell, elvish)"
        )]
        shell: Option<Shell>,
    },

    /// List servers
    Ls {
        #[arg(
            value_name = "all",
            required = false,
            help = "Display inactives (deleted) servers",
            short = 'a',
            long = "all"
        )]
        all: bool,

        #[arg(
            value_name = "long",
            required = false,
            help = "Display long format",
            short = 'l',
            long = "long"
        )]
        long: bool,
    },

    /// Describe a server
    Describe {
        #[arg(value_name = "server", help = "The name of the server to describe")]
        server: String,
    },

    /// Add a new server
    Add {
        #[arg(
            value_name = "name",
            help = "The name of the server to add",
            short = 'n',
            long = "name"
        )]
        name: String,

        #[arg(
            value_name = "host",
            help = "The host of the server to add",
            short = 'H',
            long = "host"
        )]
        host: String,

        #[arg(
            value_name = "port",
            help = "The port of the server, default is 22",
            short = 'p',
            long = "port"
        )]
        port: Option<u16>,

        #[arg(
            value_name = "user",
            help = "The user to log in the server",
            short = 'u',
            long = "user"
        )]
        user: String,

        #[arg(
            value_name = "os",
            help = "The server operating system",
            short = 'o',
            long = "os"
        )]
        os: String,

        #[arg(
            value_name = "description",
            help = "The description of the server",
            short = 'd',
            long = "description"
        )]
        description: Option<String>,

        #[arg(
            value_name = "key",
            help = "The SSH key to connect to the server",
            short = 'k',
            long = "key"
        )]
        key_path: Option<String>,
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
