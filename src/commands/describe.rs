use crate::models::Servers;

use anyhow::Result;

/// Print the details of an active server, or of every active server when `server` is `all`.
///
/// # Errors
///
/// Never fails; returns a `Result` like the other commands.
pub fn describe(servers: &Servers, server: &String) -> Result<()> {
    if server == "all" {
        servers.actives.iter().for_each(|s| println!("{s}"));
    } else {
        let server = servers.actives.iter().find(|s| s.name == *server);
        if let Some(server) = server {
            println!("{server}");
        } else {
            println!("No such resources");
        }
    }
    Ok(())
}
