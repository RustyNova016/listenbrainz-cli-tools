use moka::sync::Cache;
use serde::Deserialize;
use std::{
    env::{current_dir, current_exe},
    fs::File,
    hash::Hash,
    path::PathBuf,
};

pub mod artist_cache;
pub mod recording_cache;

pub trait CacheWrapper<K, V>
where
    K: Eq + Hash + Send + Sync + ToString + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get_cache(&self) -> &Cache<K, V>;
    fn get_cache_mut(&mut self) -> &mut Cache<K, V>;

    fn get(&self, key: &K) -> Option<V> {
        self.get_cache().get(key)
    }

    fn insert(&self, key: K, value: V) {
        self.get_cache().insert(key, value);
    }

    fn to_json_vec(&self) -> Vec<(String, V)> {
        self.get_cache()
            .iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect::<Vec<_>>()
    }

    fn load_vec(&self, value: Vec<(K, V)>) {
        for (key, value) in value {
            self.get_cache().insert(key, value);
        }
    }
}

pub trait DiskCache<'de, K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn new() -> Self;
    fn save_cache(&self) -> color_eyre::Result<()>;
    fn load_cache(&mut self) -> color_eyre::Result<()>;
    fn get_filename() -> &'static str;

    fn get_file_path() -> PathBuf;

    fn load_from_disk_or_new() -> Self;
}
