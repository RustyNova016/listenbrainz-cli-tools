use std::{collections::HashMap, fs::File};

use chrono::{DateTime, Utc};
use clap::builder::Str;
use listenbrainz::raw::response::{UserListensListen, UserListensPayload};
use moka::sync::Cache;
use serde::{Deserialize, Serialize};

use crate::{
    models::data::listens::UserListen,
    utils::{extensions::UserListensPayloadExt, println_cli},
};

use super::{CacheWrapper, DiskCache};

#[derive(Debug, Clone)]
pub struct ListenCache {
    cache: HashMap<String, UserListens>,
}

impl ListenCache {
    fn to_json_vec(&self) -> Vec<(String, UserListens)> {
        self.cache
            .clone()
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect::<Vec<_>>()
    }

    pub fn get(&self, key: &str) -> Option<&UserListens> {
        self.cache.get(key)
    }
}

impl<'de> DiskCache<'de, String, UserListens> for ListenCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn save_cache(&self) -> color_eyre::Result<()> {
        let file = File::create(Self::get_file_path())?;

        serde_json::to_writer(file, &self.to_json_vec())?;

        Ok(())
    }

    fn load_cache(&mut self) -> color_eyre::Result<()> {
        let file = File::open(Self::get_file_path())?;
        let cache_vec: Vec<(String, UserListens)> = serde_json::from_reader(file)?;

        for (key, value) in cache_vec {
            self.cache.insert(key, value);
        }

        Ok(())
    }

    fn get_filename() -> &'static str {
        todo!()
    }

    fn get_file_path() -> std::path::PathBuf {
        "C:\\test\\listens.json".into()
    }

    fn load_from_disk_or_new() -> Self {
        let mut cache = Self::new();
        let res = cache.load_cache();
        if res.is_err() {
            println_cli("Couldn't load the listen cache file. Creating a new one");
            Self::new()
        } else {
            cache
        }
    }
}

/// All of a user's listens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListens {
    username: String,
    listens: Vec<UserListenCache>,
}

impl UserListens {
    /// Remove all the listens in a specific timerange
    fn invalidate_timerange(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        self.listens.retain(|listen| {
            listen.listen_data.listened_at < start || end < listen.listen_data.listened_at
        })
    }

    pub fn insert_api_return(&mut self, data: UserListensPayload) {
        self.invalidate_timerange(data.get_oldest_listen_date(), data.get_latest_listen_date());

        for new_listen in data.listens {
            self.listens.push(new_listen.into())
        }
    }

    /// Return the listen with the latest listen date from the cache
    pub fn get_latest_cached_listen(&self) -> Option<&UserListenCache> {
        self.listens.iter().max_by_key(|listen| listen.listen_data.listened_at)
    }
}

/// An holder for a Listen with caching info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListenCache {
    pub listen_data: UserListen,
    pub updated_at: DateTime<Utc>,
}

impl UserListenCache {
    pub fn new(listen_data: UserListen) -> Self {
        Self {
            listen_data,
            updated_at: chrono::offset::Utc::now(),
        }
    }
}

impl From<UserListensListen> for UserListenCache {
    fn from(value: UserListensListen) -> Self {
        Self::new(
            value
                .try_into()
                .expect("Couldn't parse timestamp of listen"),
        )
    }
}
