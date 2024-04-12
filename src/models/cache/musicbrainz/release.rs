use crate::models::{cache::static_cache::StaticCache, data::musicbrainz::release::Release};
use cached::{DiskCacheError, IOCached};

impl StaticCache {
    pub fn insert_release(
        &self,
        key: &str,
        value: Release,
    ) -> Result<Option<Release>, DiskCacheError> {
        self.releases.set_or_update(key.to_owned(), value)
    }

    pub fn get_release(&self, key: &str) -> Result<Option<Release>, DiskCacheError> {
        self.releases.get(&key.to_string())
    }
}
