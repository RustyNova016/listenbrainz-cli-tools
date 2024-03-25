use std::{collections::HashMap, fs::File, rc::Rc};

use chrono::{DateTime, Utc};
use clap::builder::Str;
use listenbrainz::raw::response::{UserListensListen, UserListensPayload};
use moka::sync::Cache;
use serde::{Deserialize, Serialize};

use crate::{
    models::{api::listenbrainz::user_listens, data::listens::{collection::UserListenCollection, UserListen}},
    utils::{extensions::UserListensPayloadExt, println_cli},
};

use super::{CacheWrapper, DiskCache};

#[derive(Debug, Clone, Default)]
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

    pub fn get_mut(&mut self, key: &str) -> Option<&mut UserListens> {
        self.cache.get_mut(key)
    }

    pub fn get_or_new_mut(&mut self, key: &str) -> &mut UserListens {
        if self.cache.get(key).is_none() {
            self.cache
                .insert(key.to_string(), UserListens::new(key.to_string()));
        }

        self.get_mut(key)
            .expect("Could not get UserListens after insertion")
    }

    pub fn listen_count(&self) -> usize {
        self.cache.values().map(|user_listens| user_listens.listens.len()).sum()
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

        println_cli(&format!(
            "Loaded {} listens from cache",
            self.listen_count()
        ));

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
    pub fn new(username: String) -> Self {
        Self {
            username,
            listens: Vec::new(),
        }
    }

    /// Remove all the listens in a specific timerange
    fn invalidate_timerange(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        self.listens.retain(|listen| {
            listen.listen_data.listened_at < start || end < listen.listen_data.listened_at
        });
    }

    pub fn insert_api_return(&mut self, data: UserListensPayload) {
        self.invalidate_timerange(
            data.get_date_of_oldest_listen_of_payload()
                .unwrap_or(Utc::now()),
            data.get_date_of_latest_listen_of_payload()
                .unwrap_or(Utc::now()),
        );

        for new_listen in data.listens {
            self.listens.push(new_listen.into())
        }
    }

    /// Return the listen with the latest listen date from the cache
    pub fn get_latest_cached_listen(&self) -> Option<&UserListenCache> {
        self.listens
            .iter()
            .max_by_key(|listen| listen.listen_data.listened_at)
    }

    pub fn get_listens(&self) -> UserListenCollection {
        UserListenCollection::from_iter(
            self.listens
                .iter()
                .map(|cached_listen| cached_listen.listen_data.clone()),
        )
    }
}

/// An holder for a Listen with caching info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListenCache {
    pub listen_data: Rc<UserListen>,
    pub updated_at: DateTime<Utc>,
}

impl UserListenCache {
    pub fn new(listen_data: Rc<UserListen>) -> Self {
        Self {
            listen_data,
            updated_at: chrono::offset::Utc::now(),
        }
    }
}

impl From<UserListensListen> for UserListenCache {
    fn from(value: UserListensListen) -> Self {
        Self::new(Rc::new(
            UserListen::try_from(value).expect("Couldn't parse timestamp of listen"),
        ))
    }
}
