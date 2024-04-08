use std::sync::Arc;

use cached::IOCached;
use cached::{DiskCache, DiskCacheError};
use once_cell::sync::Lazy;

use crate::models::data::{
    listenbrainz::user_listens::UserListens,
    recording::{Artist, Recording},
};

use super::CACHE_LOCATION;

pub(crate) static STATIC_CACHE: Lazy<Arc<StaticCache>> = Lazy::new(|| Arc::new(StaticCache::new()));

pub struct StaticCache {
    // MusicBrainz Caches
    pub(super) recordings: Lazy<DiskCache<String, Recording>>,
    pub(super) artists: Lazy<DiskCache<String, Artist>>,

    // Listenbrainz Caches
    pub(super) listens: Lazy<DiskCache<String, UserListens>>,
}

impl StaticCache {
    pub fn new() -> Self {
        Self {
            recordings: Lazy::new(|| {
                DiskCache::new("recordings")
                    .set_disk_directory(CACHE_LOCATION.clone())
                    .build()
                    .unwrap()
            }),
            artists: Lazy::new(|| {
                DiskCache::new("artists")
                    .set_disk_directory(CACHE_LOCATION.clone())
                    .build()
                    .unwrap()
            }),

            listens: Lazy::new(|| {
                DiskCache::new("listens")
                    .set_disk_directory(CACHE_LOCATION.clone())
                    .build()
                    .unwrap()
            }),
        }
    }

    pub fn get_artist(&self, key: &str) -> Result<Option<Artist>, DiskCacheError> {
        self.artists.cache_get(&key.to_string())
    }

    pub fn get_recording(&self, key: &str) -> Result<Option<Recording>, DiskCacheError> {
        self.recordings.cache_get(&key.to_string())
    }

    pub fn insert_recording(
        &self,
        key: String,
        value: Recording,
    ) -> Result<Option<Recording>, DiskCacheError> {
        self.recordings.cache_set(key, value)
    }

    pub fn insert_artist(
        &self,
        key: String,
        value: Artist,
    ) -> Result<Option<Artist>, DiskCacheError> {
        self.artists.cache_set(key, value)
    }
}

impl Default for StaticCache {
    fn default() -> Self {
        Self::new()
    }
}
