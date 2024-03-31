use std::ops::Deref;
use std::path::Path;

use color_eyre::owo_colors::OwoColorize;
use listenbrainz::raw::response::UserListensPayload;
use moka::sync::Cache;
use once_cell::sync::Lazy;

use crate::{
    models::cache::{CacheWrapper, DiskCache},
    utils::println_cli,
};

use super::UserListensCache;

pub(crate) static LISTEN_CACHE: Lazy<Cache<String, UserListensCache>> =
    Lazy::new(create_listen_cache);

fn create_listen_cache() -> Cache<String, UserListensCache> {
    let Ok(cache) = ListenCache::try_load_new_cache() else {
        println_cli("Failed to load listen cache. A new one will get created".red());

        return ListenCache::get_base_cache();
    };

    cache.run_pending_tasks();
    println_cli(format!("Loaded {} listen from cache", cache.entry_count()));

    cache
}

#[derive(Debug, Clone)]
pub struct ListenCache {
    cache: Cache<String, UserListensCache>,
}

impl ListenCache {
    pub fn listen_count(&self) -> usize {
        self.cache
            .iter()
            .map(|(_, user_listens)| user_listens.listens.len())
            .sum()
    }

    fn get_or_new(&self, key: String) -> UserListensCache {
        self.get(&key)
            .unwrap_or_else(|| UserListensCache::new(key.to_string()))
    }

    /// Insert a listen payload into the cache. It will overwrite any existing listens
    pub fn insert_listen_payload(&self, username: &str, listens: UserListensPayload) {
        let mut user = self.get_or_new(username.to_string());

        user.insert_api_return(listens);

        // Update the cache with the new data
        self.insert(username.to_string(), user);
    }
}

impl Default for ListenCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ListenCache {
    type Target = Cache<String, UserListensCache>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl CacheWrapper<String, UserListensCache> for ListenCache {}

impl DiskCache<String, UserListensCache> for ListenCache {
    fn new() -> Self {
        ListenCache {
            cache: LISTEN_CACHE.clone(),
        }
    }

    fn get_filename() -> &'static Path {
        Path::new("listens.json")
    }

    fn get_static_cache() -> &'static Lazy<Cache<String, UserListensCache>> {
        &LISTEN_CACHE
    }
}
