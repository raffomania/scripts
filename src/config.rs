use std::{fs::File, io::Read, path::PathBuf};

use lettre::message::Mailbox;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub smtp: Smtp,
    pub mail: Mail,
}

#[derive(Deserialize, Debug)]
pub struct Mail {
    pub from: Mailbox,
}

#[derive(Deserialize, Debug)]
pub struct Smtp {
    pub host: String,
    pub user: String,
    pub port: u32,
    pub pass_command: String,
}

fn config_path() -> PathBuf {
    dirs::config_dir().unwrap().join("scripts/scripts.toml")
}

pub fn read() -> Config {
    let mut body = String::new();
    File::open(config_path())
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();

    toml::from_str(&body).unwrap()
}
