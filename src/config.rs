use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub remote: Remote,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Remote {
    pub name: String,
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        let config_content = read_to_string(&file_path).unwrap_or_else(|_| {
            panic!(
                "Couldn't open `{}` configuration file. Is it missing?",
                &file_path
            )
        });

        let config: Config = toml::from_str(&config_content).unwrap_or_else(|_| {
            panic!(
                "Couldn't extract config from `{}`. Is it properly settled?",
                &file_path
            )
        });

        config
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Remote};
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    #[should_panic(
        expected = "Couldn't open `some_invalid_path.toml` configuration file. Is it missing?"
    )]
    fn no_config_file_found() {
        Config::new("some_invalid_path.toml");
    }

    #[test]
    #[should_panic(expected = "Couldn't extract config from")]
    fn invalid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("fake.toml");
        let mut config_file = File::create(file_path.clone()).unwrap();

        if let Err(_) = writeln!(config_file, "[remote]\nnam = \"fake\"") {
            panic!("Oops. Something bad happened with invalid_file().")
        }

        Config::new(file_path.to_str().unwrap());
        drop(config_file);
        dir.close().unwrap();
    }

    #[test]
    fn valid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("fake.toml");
        let mut config_file = File::create(file_path.clone()).unwrap();

        if let Err(_) = writeln!(config_file, "[remote]\nname = \"fake\"") {
            panic!("Oops. Something bad happened with valid_file().")
        }

        let config = Config::new(file_path.to_str().unwrap());

        assert_eq!(
            config,
            Config {
                remote: Remote {
                    name: "fake".to_string()
                }
            }
        );

        drop(config_file);
        dir.close().unwrap();
    }
}
