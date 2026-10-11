use clap::{CommandFactory, Parser, error::ErrorKind};
use clap_complete::Shell;
use oh_my_servers::cli::{Args, Commands};

fn parse(args: &[&str]) -> Commands {
    Args::try_parse_from(std::iter::once("oh-my-servers").chain(args.iter().copied()))
        .unwrap()
        .command
}

fn parse_err(args: &[&str]) -> ErrorKind {
    Args::try_parse_from(std::iter::once("oh-my-servers").chain(args.iter().copied()))
        .err()
        .unwrap()
        .kind()
}

#[test]
fn cli_definition_is_valid() {
    Args::command().debug_assert();
}

#[test]
fn shell() {
    assert!(matches!(parse(&["shell"]), Commands::Shell {}));
}

#[test]
fn ls_defaults() {
    assert!(matches!(
        parse(&["ls"]),
        Commands::Ls {
            all: false,
            long: false
        }
    ));
}

#[test]
fn ls_flags() {
    assert!(matches!(
        parse(&["ls", "-a", "-l"]),
        Commands::Ls {
            all: true,
            long: true
        }
    ));
    assert!(matches!(
        parse(&["ls", "--all", "--long"]),
        Commands::Ls {
            all: true,
            long: true
        }
    ));
}

#[test]
fn describe() {
    let Commands::Describe { server } = parse(&["describe", "alpha"]) else {
        panic!("expected describe");
    };
    assert_eq!(server, "alpha");
}

#[test]
fn describe_requires_server() {
    assert_eq!(
        parse_err(&["describe"]),
        ErrorKind::MissingRequiredArgument
    );
}

#[test]
fn completion() {
    assert!(matches!(
        parse(&["completion", "zsh"]),
        Commands::Completion {
            shell: Some(Shell::Zsh)
        }
    ));
    assert!(matches!(
        parse(&["completion"]),
        Commands::Completion { shell: None }
    ));
}

#[test]
fn completion_rejects_unknown_shell() {
    assert_eq!(parse_err(&["completion", "cmd"]), ErrorKind::InvalidValue);
}

#[test]
fn add_all_options() {
    let Commands::Add {
        name,
        host,
        port,
        user,
        os,
        description,
        key_path,
    } = parse(&[
        "add", "-n", "web", "-H", "10.0.0.1", "-p", "2222", "-u", "root", "-o", "debian", "-d",
        "Web server", "-k", "~/.ssh/web",
    ])
    else {
        panic!("expected add");
    };

    assert_eq!(name, "web");
    assert_eq!(host, "10.0.0.1");
    assert_eq!(port, Some(2222));
    assert_eq!(user, "root");
    assert_eq!(os, "debian");
    assert_eq!(description.as_deref(), Some("Web server"));
    assert_eq!(key_path.as_deref(), Some("~/.ssh/web"));
}

#[test]
fn add_optional_options() {
    let Commands::Add {
        port,
        description,
        key_path,
        ..
    } = parse(&[
        "add", "--name", "web", "--host", "h", "--user", "u", "--os", "o",
    ])
    else {
        panic!("expected add");
    };

    assert_eq!(port, None);
    assert_eq!(description, None);
    assert_eq!(key_path, None);
}

#[test]
fn add_requires_mandatory_options() {
    assert_eq!(
        parse_err(&["add", "-n", "web", "-H", "h", "-u", "u"]),
        ErrorKind::MissingRequiredArgument
    );
}

#[test]
fn add_rejects_invalid_port() {
    for port in ["abc", "70000", "-1"] {
        assert_ne!(
            parse_err(&["add", "-n", "w", "-H", "h", "-u", "u", "-o", "o", "-p", port]),
            ErrorKind::MissingRequiredArgument,
            "port {port}"
        );
    }
}

#[test]
fn delete() {
    let Commands::Delete {
        server,
        permanently,
    } = parse(&["delete", "alpha"])
    else {
        panic!("expected delete");
    };
    assert_eq!(server, "alpha");
    assert!(!permanently);

    assert!(matches!(
        parse(&["delete", "alpha", "--permanently"]),
        Commands::Delete {
            permanently: true,
            ..
        }
    ));
}

#[test]
fn missing_subcommand() {
    assert_eq!(
        parse_err(&[]),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn unknown_subcommand() {
    assert_eq!(parse_err(&["nope"]), ErrorKind::InvalidSubcommand);
}
