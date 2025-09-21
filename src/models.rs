use colored::Colorize;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, option::Option};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub os: String,
    pub description: String,
    pub key_path: Option<String>,
    pub password: Option<String>,
}

impl Server {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &String,
        host: &String,
        port: Option<u16>,
        user: &String,
        os: &String,
        description: Option<String>,
        key_path: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            host: host.to_owned(),
            port,
            user: user.to_owned(),
            os: os.to_owned(),
            description: description.unwrap_or("No description".to_string()),
            key_path,
            password,
        }
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!(
            "{}\n\
             \tHost: {}\n\
             \tPort: {}\n\
             \tUser: {}\n\
             \tOS: {}\n\
             \tDescription: {}\n\
             \tPassword: {}\n\
             \tKey Path: {}",
            self.name.underline(),
            self.host,
            self.port.unwrap_or(22),
            self.user,
            self.os,
            self.description,
            self.password.as_deref().unwrap_or("No password"),
            self.key_path.as_deref().unwrap_or("No key"),
        );

        write!(f, "{output}")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Servers {
    pub actives: Vec<Server>,
    pub inactives: Vec<Server>,
}
