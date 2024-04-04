use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::{fs::File, hash::Hash, ops::Deref};

use directories::BaseDirs;
use itertools::Itertools;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

use self::{
    artist_cache::ArtistCache, listen_cache::cache::ListenCache, recording_cache::RecordingCache,
};

pub mod artist_cache;
pub mod disk_cache;
pub mod global_cache;
pub mod listen_cache;
pub mod recording_cache;
pub mod static_cache;

pub static CACHE_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find standard directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();
    path.push("listenbrainz_cli_tools");
    path
});

pub trait CacheWrapper<K, V>: Deref<Target = Cache<K, V>>
where
    K: Eq + Hash + Send + Sync + ToString + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn load_vec(&self, value: Vec<(K, V)>) {
        for (key, value) in value {
            self.insert(key, value);
        }
    }
}

/// Trait for any cache that can be saved to disk
/// CK and DK, as well as CV and DV are the same values. Limitations in the borrow checker prevent a generic 'static element to be passed as a type that also implement Deserialize.
/// So as a workarround, we split the two types, linking them by a From<>
pub trait DiskCache<K, V>: Deref<Target = Cache<K, V>>
where
    K: Hash + Eq + Send + Sync + ToString + 'static,
    V: Serialize + DeserializeOwned + Clone + Send + Sync + Debug + 'static,
{
    fn new() -> Self;

    fn get_filename() -> &'static Path;

    fn get_file_path() -> PathBuf {
        let mut path = CACHE_LOCATION.clone();
        path.push(Self::get_filename());
        path
    }

    /// Return the base cache to use for this cache
    fn get_base_cache() -> Cache<String, V> {
        Cache::builder().max_capacity(100000).build()
    }

    fn try_load_new_cache() -> color_eyre::Result<Cache<String, V>> {
        let cache: Cache<String, V> = Self::get_base_cache();

        let cache_file = File::open(Self::get_file_path())?;
        let cache_data: Vec<(String, V)> = serde_json::from_reader(cache_file)?;

        for (key, value) in cache_data {
            cache.insert(key, value);
        }

        Ok(cache)
    }

    fn to_json_vec(&self) -> Vec<(String, V)> {
        self.iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect_vec()
    }

    fn save_cache(&self) -> color_eyre::Result<()> {
        let file = File::create(Self::get_file_path())?;

        let json_vec = self.to_json_vec();
        serde_json::to_writer(file, &json_vec)?;

        Ok(())
    }

    fn get_static_cache() -> &'static Lazy<Cache<K, V>>;

    /// Save the cache only if it is loaded
    fn save_if_loaded() -> color_eyre::Result<()>
    where
        Self: Sized,
    {
        let lazy_cache = Self::get_static_cache();

        if Lazy::get(lazy_cache).is_some() {
            return Self::new().save_cache();
        }

        Ok(())
    }
}

pub fn global_cache_save() -> color_eyre::Result<()> {
    // Musicbrainz
    ArtistCache::save_if_loaded()?;
    RecordingCache::save_if_loaded()?;

    // Listenbrainz
    ListenCache::save_if_loaded()?;

    Ok(())
}
