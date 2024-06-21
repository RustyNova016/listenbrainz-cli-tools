pub mod mapper;
pub mod recording_timeout;
use crate::core::entity_traits::config_file::ConfigFile;
use derive_getters::Getters;
use mapper::MapperConfig;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
pub struct Config {
    /// Saved usertokens
    tokens: HashMap<String, String>,

    pub(super) mapper: Option<MapperConfig>,
}

impl Config {
    pub fn set_token(&mut self, username: String, token: String) {
        self.tokens.insert(username.to_lowercase(), token);
    }

    pub fn get_token_or_argument(username: &str, arg: &Option<String>) -> String {
        if let Some(arg) = arg {
            return arg.clone();
        }

        let config = Self::load().unwrap();
        if let Some(token) = config.tokens.get(&username.to_lowercase()) {
            return token.clone();
        }

        panic!("No token was provided. To properly run, this command need an user token. Either provide one or add it to the configuration file using `config set-token <USER> <TOKEN>`")
    }
}

impl ConfigFile for Config {
    fn file_name() -> &'static str {
        "config.json"
    }
}
