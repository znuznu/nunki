use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct Config {
    pub remote: Remote,
}

#[derive(Deserialize)]
pub struct Remote {
    pub name: String,
}

impl Config {
    pub fn new() -> Self {
        let config_content = read_to_string("nunki.toml")
            .expect("Couldn't open `nunki.toml` configuration file. Is it missing?");

        let config: Config = toml::from_str(&config_content)
            .expect("Couldn't extract config from `nunki.toml`. Is it properly settled?");

        config
    }
}
