use crate::models::Servers;
use anyhow::Result;
use std::fmt::Write as _;

pub fn generate_aliases(servers: &Servers) -> Result<()> {
    let mut aliases = String::new();
    servers.actives.iter().for_each(|server| {
        let s_name = &server.name;
        let s_user = &server.user;
        let s_host = &server.host;

        let _ = if server.key_path.is_some() {
            let s_key = &server.key_path.as_ref().expect("Already checked above");
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
