use crate::models::Servers;
use anyhow::Result;
use colored::Colorize;

pub fn ls(servers: &Servers, all: bool) -> Result<()> {
    if all {
        println!("{}", "actives".green());
        servers
            .actives
            .iter()
            .for_each(|srv| println!("{}:\t {}", srv.name.underline(), srv.description));

        println!("{}", "inactives".red().italic());
        servers.inactives.iter().for_each(|srv| {
            println!(
                "{}:\t {}",
                srv.name.underline().italic(),
                srv.description.italic()
            )
        });
    } else {
        servers
            .actives
            .iter()
            .for_each(|serv| println!("{}:\t {}", serv.name.underline(), serv.description));
    }
    Ok(())
}
