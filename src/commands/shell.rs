use crate::models::Servers;
use anyhow::Result;
use std::fmt::Write as _;

/// Print a shell `alias` for each active server.
///
/// # Errors
///
/// Never fails; returns a `Result` like the other commands.
pub fn generate_aliases(servers: &Servers) -> Result<()> {
    let mut aliases = String::new();
    servers.actives.iter().for_each(|server| {
        let s_name = &server.name;
        let s_user = &server.user;
        let s_host = &server.host;

        let _ = if let Some(s_key) = &server.key_path {
            writeln!(
                &mut aliases,
                "alias {s_name}='ssh {s_user}@{s_host} -i {s_key}'"
            )
        } else {
            writeln!(&mut aliases, "alias {s_name}='ssh {s_user}@{s_host}'")
        };
    });
    println!("{aliases}");
    Ok(())
}
