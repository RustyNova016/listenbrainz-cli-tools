use std::fmt::Display;
use std::sync::Arc;

use cached::{DiskCache, DiskCacheError, IOCached};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;

use super::disk_cache::DiskCacheWrapper;
use super::CACHE_LOCATION;

pub(crate) static STATIC_CACHE: Lazy<Arc<StaticCache>> = Lazy::new(|| Arc::new(StaticCache::new()));

pub struct StaticCache {
    // MusicBrainz Caches
    pub(super) artists: Lazy<DiskCacheWrapper<String, Artist>>,
    pub(super) recordings: Lazy<DiskCacheWrapper<String, Recording>>,
    pub(super) releases: Lazy<DiskCacheWrapper<String, Release>>,

    // Listenbrainz Caches
    pub(super) listens: Lazy<DiskCacheWrapper<String, UserListens>>,
}

fn create_cache<K, V>(name: &str) -> DiskCache<K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    DiskCache::new(name)
        .set_disk_directory(CACHE_LOCATION.clone())
        .build()
        .unwrap()
}

impl StaticCache {
    pub fn new() -> Self {
        Self {
            recordings: Lazy::new(|| DiskCacheWrapper::new("recordings")),
            artists: Lazy::new(|| DiskCacheWrapper::new("artists")),
            releases: Lazy::new(|| DiskCacheWrapper::new("releases")),

            listens: Lazy::new(|| DiskCacheWrapper::new("listens")),
        }
    }

    pub fn get_artist(&self, key: &str) -> Result<Option<Artist>, DiskCacheError> {
        self.artists.get(&key.to_string())
    }

    pub fn get_recording(&self, key: &str) -> Result<Option<Recording>, DiskCacheError> {
        self.recordings.get(&key.to_string())
    }

    pub fn insert_recording(
        &self,
        key: String,
        value: Recording,
    ) -> Result<Option<Recording>, DiskCacheError> {
        self.recordings.set_or_update(key, value)
    }

    pub fn insert_artist(
        &self,
        key: String,
        value: Artist,
    ) -> Result<Option<Artist>, DiskCacheError> {
        self.artists.set_or_update(key, value)
    }
}

impl Default for StaticCache {
    fn default() -> Self {
        Self::new()
    }
}
