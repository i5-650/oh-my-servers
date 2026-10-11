use oh_my_servers::models::{Server, Servers};

fn no_color() {
    colored::control::set_override(false);
}

#[test]
fn new_fills_every_field() {
    let server = Server::new(
        &"prod".to_string(),
        &"10.0.0.1".to_string(),
        Some(2222),
        &"admin".to_string(),
        &"Ubuntu".to_string(),
        Some("Production server".to_string()),
        Some("~/.ssh/prod".to_string()),
    );

    assert_eq!(server.name, "prod");
    assert_eq!(server.host, "10.0.0.1");
    assert_eq!(server.port, Some(2222));
    assert_eq!(server.user, "admin");
    assert_eq!(server.os, "Ubuntu");
    assert_eq!(server.description, "Production server");
    assert_eq!(server.key_path.as_deref(), Some("~/.ssh/prod"));
}

#[test]
fn new_uses_default_description() {
    let server = Server::new(
        &"a".to_string(),
        &"h".to_string(),
        None,
        &"u".to_string(),
        &"os".to_string(),
        None,
        None,
    );
    assert_eq!(server.description, "No description");
}

#[test]
fn display_output() {
    no_color();
    let server = Server::new(
        &"prod".to_string(),
        &"10.0.0.1".to_string(),
        Some(2222),
        &"admin".to_string(),
        &"Ubuntu".to_string(),
        Some("Production server".to_string()),
        None,
    );

    assert_eq!(
        format!("{server}"),
        "prod\n\
         \tHost: 10.0.0.1\n\
         \tPort: 2222\n\
         \tUser: admin\n\
         \tOS: Ubuntu\n\
         \tDescription: Production server\n\
         \tAuthentication: password"
    );
}

#[test]
fn display_defaults_to_port_22() {
    no_color();
    let server = Server::new(
        &"default".to_string(),
        &"localhost".to_string(),
        None,
        &"me".to_string(),
        &"Arch".to_string(),
        None,
        None,
    );

    let output = format!("{server}");
    assert!(output.contains("Port: 22\n"));
    assert!(output.contains("Description: No description"));
    assert!(output.contains("Authentication: password"));
}

#[test]
fn display_ssh_key_authentication() {
    no_color();
    let server = Server::new(
        &"k".to_string(),
        &"h".to_string(),
        None,
        &"u".to_string(),
        &"os".to_string(),
        None,
        Some("~/.ssh/id_ed25519".to_string()),
    );

    assert!(format!("{server}").ends_with("Authentication: ssh key ~/.ssh/id_ed25519"));
}

#[test]
fn serde_uses_camel_case() {
    let json = r#"{
        "actives": [{
            "name": "a", "host": "h", "port": 2200, "user": "u", "os": "o",
            "description": "d", "keyPath": "/k"
        }],
        "inactives": []
    }"#;

    let servers: Servers = serde_json::from_str(json).unwrap();
    assert_eq!(servers.actives[0].key_path.as_deref(), Some("/k"));
    assert_eq!(servers.actives[0].port, Some(2200));

    let back = serde_json::to_string(&servers).unwrap();
    assert!(back.contains("\"keyPath\":\"/k\""));
}

#[test]
fn serde_optional_fields_may_be_missing() {
    let yaml = "
actives:
  - name: a
    host: h
    user: u
    os: o
    description: d
inactives: []
";
    let servers: Servers = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(servers.actives[0].port, None);
    assert_eq!(servers.actives[0].key_path, None);
}

#[test]
fn serde_rejects_missing_required_field() {
    let yaml = "
actives:
  - name: a
    user: u
    os: o
    description: d
inactives: []
";
    assert!(serde_yaml::from_str::<Servers>(yaml).is_err());
}

#[test]
fn yaml_round_trip() {
    let servers = Servers {
        actives: vec![Server::new(
            &"a".to_string(),
            &"h".to_string(),
            Some(22),
            &"u".to_string(),
            &"o".to_string(),
            None,
            Some("/k".to_string()),
        )],
        inactives: vec![],
    };

    let yaml = serde_yaml::to_string(&servers).unwrap();
    assert_eq!(serde_yaml::from_str::<Servers>(&yaml).unwrap(), servers);
}
