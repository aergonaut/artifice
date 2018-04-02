use std;
use std::io::prelude::*;

use failure::Error;
use failure::err_msg;
use toml;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Config {
    pub jira: JiraConfig
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct JiraConfig {
    pub username: String,
    pub password: String
}

impl Config {
    pub fn get() -> Result<Config, Error> {
        if let Some(home_dir) = std::env::home_dir() {
            let default_config = home_dir.join(".artifice.toml");
            let mut buffer = String::new();
            let mut config_file = std::fs::File::open(default_config)?;
            config_file.read_to_string(&mut buffer)?;

            let config = toml::from_str(&buffer)?;
            return Ok(config)
        }
        Err(err_msg("Couldn't read default config"))
    }

    fn default_config() -> Result<Config, Error> {
        let home_dir = std::env::home_dir().ok_or(err_msg("Could not find user's home directory"))?;
        let default_config = home_dir.join(".artifice.toml");
        let mut buffer = String::new();
        let mut config_file = std::fs::File::open(default_config)?;
        config_file.read_to_string(&mut buffer)?;

        let config = toml::from_str(&buffer)?;
        Ok(config)
    }
}
