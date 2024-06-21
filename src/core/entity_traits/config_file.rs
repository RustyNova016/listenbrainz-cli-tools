use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

use directories::BaseDirs;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::error::Error;

pub trait ConfigFile: Serialize + DeserializeOwned + Default {
    fn file_name() -> &'static str;

    fn path_to_config() -> PathBuf {
        let mut path = BaseDirs::new()
            .expect("Couldn't find standard directory. Is your system an oddball one?")
            .config_dir()
            .to_path_buf();
        path.push("listenbrainz_cli_tools");
        fs::create_dir_all(&path).expect("Couldn't create config directory");
        path.push(Self::file_name());
        path
    }

    fn save(&self) -> Result<(), Error> {
        let config_file = File::create(Self::path_to_config().as_path())
            .map_err(Error::ConfigFileCreationError)?;
        serde_json::to_writer_pretty(config_file, self).map_err(Error::ConfigFileWriteError)?;
        Ok(())
    }

    fn load() -> Result<Self, Error> {
        match Self::get_config_reader() {
            Ok(Some(data)) => {
                serde_json::from_reader(data).map_err(Error::ConfigLoadDeserializationError)
            }
            Ok(None) => Ok(Self::default()),
            Err(err) => Err(Error::ConfigLoadError(err)),
        }
    }

    fn get_config_reader() -> io::Result<Option<File>> {
        match File::open(Self::path_to_config().as_path()) {
            Ok(reader) => Ok(Some(reader)),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(None),
                _ => Err(err),
            },
        }
    }
}
