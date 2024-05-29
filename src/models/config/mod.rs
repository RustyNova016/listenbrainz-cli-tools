pub mod self_edits;
use derive_getters::Getters;
use self_edits::SelfEdit;
use self_edits::SelfEditAction;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

use crate::core::caching::CONFIG_FILE;

use super::cli::config::SelfEditActionValue;
use super::cli::config::SelfEditType;
use super::data::musicbrainz::mbid::MBID;
use super::data::musicbrainz::recording::mbid::RecordingMBID;
use super::error::Error;

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
pub struct Config {
    /// Saved usertokens
    tokens: HashMap<String, String>,
    self_edits: HashMap<MBID, SelfEdit>
}

impl Config {
    pub fn set_token(&mut self, username: String, token: String) {
        self.tokens.insert(username, token);
    }

    pub fn get_token_or_argument(username: &str, arg: &Option<String>) -> String {
        if let Some(arg) = arg {
            return arg.clone();
        }

        let config = Self::load().unwrap();
        if let Some(token) = config.tokens.get(username) {
            return token.clone();
        }

        panic!("No token was provided. To properly run, this command need an user token.")
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

    pub fn load() -> Result<Self, Error> {
        match Self::get_config_reader() {
            Ok(Some(data)) => {
                serde_json::from_reader(data).map_err(Error::ConfigLoadDeserializationError)
            }
            Ok(None) => Ok(Self::default()),
            Err(err) => Err(Error::ConfigLoadError(err)),
        }
    }

    pub fn set_edit(edited_mbid: String, on: SelfEditType, action: SelfEditActionValue, edit_target: Option<String>)  {
        let mut config = Self::load().unwrap();
        let edited_mbid = MBID::Recording(edited_mbid.into());
        let mut edit = config.self_edits.get(&edited_mbid).cloned().unwrap_or_default();

        edit.set_action(on, action, edit_target.map(RecordingMBID::from).map(MBID::Recording));

        config.self_edits.insert(edited_mbid, edit);
        config.save();
    }
}
