use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

use crate::core::caching::CONFIG_FILE;

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
pub struct Config {
    /// Saved usertokens
    tokens: HashMap<String, String>,
}

impl Config {
    pub fn set_token(&mut self, username: String, token: String) {
        self.tokens.insert(username, token);
    }

    pub fn save(&self) -> color_eyre::Result<()> {
        let config_file = File::create(CONFIG_FILE.as_path())?;
        serde_json::to_writer_pretty(config_file, self)?;
        Ok(())
    }

    fn get_config_reader() -> io::Result<Option<File>> {
        match File::open(CONFIG_FILE.as_path()) {
            Ok(reader) => Ok(Some(reader)),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(None),
                _ => Err(err),
            },
        }
    }

    pub fn load() -> color_eyre::Result<Self> {
        if let Some(data) = Self::get_config_reader()? {
            Ok(serde_json::from_reader(data)?)
        } else {
            Ok(Self::default())
        }
    }
}
