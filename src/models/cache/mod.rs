use std::fs::File;
use moka::sync::Cache;
use std::hash::Hash;
use serde::Deserialize;
use crate::models::data::recording::Recording;
use crate::utils::println_cli;

mod artist_cache;
pub mod recording_cache;

trait CacheWrapper<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get_cache(&self) -> &Cache<K, V>;
    fn get_cache_mut(&mut self) -> &mut Cache<K, V>;
    
    fn get(&self, key: &K) -> Option<V> {
        self.get_cache().get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        self.get_cache().insert(key, value);
    }
}

trait DiskCache<'de, K, V> where
K: Deserialize<'de>, V: Deserialize<'de> {
    fn save_cache(&self) -> color_eyre::Result<()>;
    fn load_cache(&mut self) -> color_eyre::Result<()>;
}