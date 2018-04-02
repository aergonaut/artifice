use std;
use std::io::prelude::*;
use std::path::{ PathBuf };

use failure::{ Error, ResultExt };
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
    /// Open a config file at the given path.
    pub fn open(path: &Option<PathBuf>) -> Result<Config, Error> {
        let config_path = match path {
            Some(path) => path.clone(),
            None => Config::default_config()?
        };
        let mut config_file = std::fs::File::open(config_path.clone()).context(format!("Could not open {:?}", config_path))?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        let config = toml::from_str(&buffer)?;
        Ok(config)
    }

    /// Open the default config. The default config is named `.artifice.toml` and is located in the
    /// user's home directory.
    pub fn default_config() -> Result<PathBuf, Error> {
        let home_dir = std::env::home_dir().ok_or(err_msg("Could not find user's home directory"))?;
        let default_config = home_dir.join(".artifice.toml");
        Ok(default_config)
    }
}
