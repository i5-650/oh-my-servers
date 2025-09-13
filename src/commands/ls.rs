use crate::models::Servers;
use colored::Colorize;

pub fn ls(servers: &Servers) {
    servers
        .actives
        .iter()
        .for_each(|serv| println!("{}:\t {}", serv.name.underline(), serv.description));
}
