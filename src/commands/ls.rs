use crate::models::{Server, Servers};
use anyhow::Result;
use colored::Colorize;

pub fn ls(servers: &Servers, all: bool, long: bool) -> Result<()> {
    if all {
        println!("{}", "actives".green());
        servers
            .actives
            .iter()
            .for_each(|srv| display_server(srv, long));

        println!("{}", "inactives".red().italic());
        servers
            .inactives
            .iter()
            .for_each(|srv| display_server(srv, long));
    } else {
        servers
            .actives
            .iter()
            .for_each(|srv| display_server(srv, long));
    }
    Ok(())
}

fn display_server(srv: &Server, long: bool) {
    if long {
        println!("{srv}");
    } else {
        println!("{}:\t {}", srv.name.underline(), srv.description);
    }
}
