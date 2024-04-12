use std::sync::Arc;

use cached::DiskCacheError;
use once_cell::sync::Lazy;

use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;

use super::disk_cache::DiskCacheWrapper;

pub(crate) static STATIC_CACHE: Lazy<Arc<StaticCache>> = Lazy::new(|| Arc::new(StaticCache::new()));

pub struct StaticCache {
    // MusicBrainz Caches
    pub(super) artists: Lazy<Arc<DiskCacheWrapper<String, Artist>>>,
    pub(super) recordings: Lazy<Arc<DiskCacheWrapper<String, Recording>>>,
    pub(super) releases: Lazy<Arc<DiskCacheWrapper<String, Release>>>,

    // Listenbrainz Caches
    pub(super) listens: Lazy<Arc<DiskCacheWrapper<String, UserListens>>>,
}

impl StaticCache {
    pub fn new() -> Self {
        Self {
            recordings: Lazy::new(|| Arc::new(DiskCacheWrapper::new("recordings"))),
            artists: Lazy::new(|| Arc::new(DiskCacheWrapper::new("artists"))),
            releases: Lazy::new(|| Arc::new(DiskCacheWrapper::new("releases"))),

            listens: Lazy::new(|| Arc::new(DiskCacheWrapper::new("listens"))),
        }
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

    pub fn get_artist_cache(&self) -> Arc<DiskCacheWrapper<String, Artist>> {
        self.artists.clone()
    }

    pub fn get_recording_cache(&self) -> Arc<DiskCacheWrapper<String, Recording>> {
        self.recordings.clone()
    }

    pub fn get_release_cache(&self) -> Arc<DiskCacheWrapper<String, Release>> {
        self.releases.clone()
    }

    pub fn get_listen_cache(&self) -> Arc<DiskCacheWrapper<String, UserListens>> {
        self.listens.clone()
    }
}

impl Default for StaticCache {
    fn default() -> Self {
        Self::new()
    }
}
