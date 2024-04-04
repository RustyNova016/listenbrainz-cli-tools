use once_cell::sync::Lazy;
use std::sync::{atomic::AtomicU64, Arc};

use crate::models::data::recording::{Artist, Recording};

use super::{disk_cache::DiskCache, listen_cache::UserListensCache};

pub(crate) static STATIC_CACHE: Lazy<Arc<StaticCache>> = Lazy::new(||Arc::new(StaticCache::new()));

#[derive(Debug)]
pub struct StaticCache {
    // MusicBrainz Caches
    recordings: Lazy<DiskCache<String, Recording>>,
    artists: Lazy<DiskCache<String, Artist>>,

    // Listenbrainz Caches
    listens: Lazy<DiskCache<String, UserListensCache>>,

    // Data
    insertion_count: AtomicU64
}

impl StaticCache {
    pub fn new() -> Self {
        Self {
            recordings: Lazy::new(|| DiskCache::load_or_new("recordings.json".to_string())),
            artists: Lazy::new(|| DiskCache::load_or_new("artists.json".to_string())),

            listens: Lazy::new(|| DiskCache::load_or_new("listens.json".to_string())),

            insertion_count: AtomicU64::new(0)
        }
    }

    pub fn get_artist(&self, key: &str) -> Option<Arc<Artist>> {
        self.artists.get(&key.to_string())
    }

    pub fn get_listen(&self, key: &str) -> Option<Arc<UserListensCache>> {
        self.listens.get(&key.to_string())
    }

    pub fn get_recording(&self, key: &str) -> Option<Arc<Recording>> {
        self.recordings.get(&key.to_string())
    }

    pub fn insert_recording(
        &self,
        key: Arc<String>,
        value: Arc<Recording>,
    ) -> Option<Arc<Recording>> {
        self.recordings.insert(key, value)
    }

    pub fn insert_artist(&self, key: Arc<String>, value: Arc<Artist>) -> Option<Arc<Artist>> {
        self.artists.insert(key, value)
    }
}

impl Default for StaticCache {
    fn default() -> Self {
        Self::new()
    }
}