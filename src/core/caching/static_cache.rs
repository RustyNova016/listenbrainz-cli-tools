use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::models::data::listenbrainz::user_listens::UserListens;
use super::disk_cache::DiskCacheWrapper;

pub(crate) static STATIC_CACHE: Lazy<Arc<StaticCache>> = Lazy::new(|| Arc::new(StaticCache::new()));

pub struct StaticCache {
    // MusicBrainz Caches

    // Listenbrainz Caches
    pub(crate) listens: Lazy<Arc<DiskCacheWrapper<String, UserListens>>>,
}

impl StaticCache {
    pub fn new() -> Self {
        Self {
            listens: Lazy::new(|| Arc::new(DiskCacheWrapper::new("listens"))),
        }
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
