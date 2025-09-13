use crate::models::Servers;

pub fn describe(servers: &Servers, server: &String) {
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
}
