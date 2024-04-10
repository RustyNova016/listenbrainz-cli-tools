use std::fmt::Display;

use cached::{DiskCache, DiskCacheError, IOCached};
use serde::{de::DeserializeOwned, Serialize};

use super::{traits::merge::UpdateCachedEntity, CACHE_LOCATION};

pub struct DiskCacheWrapper<K, V> {
    cache: DiskCache<K, V>,
}

impl<K, V> DiskCacheWrapper<K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned + UpdateCachedEntity,
{
    pub fn new(name: &str) -> Self {
        Self {
            cache: DiskCache::new(name)
                .set_disk_directory(CACHE_LOCATION.clone())
                .build()
                .unwrap(),
        }
    }

    pub fn set(&self, key: K, value: V) -> Result<Option<V>, DiskCacheError> {
        self.cache.cache_set(key, value)
    }

    pub fn set_or_update(&self, key: K, value: V) -> Result<Option<V>, DiskCacheError> {
        let cached = self.cache.cache_get(&key)?;

        let new;
        if let Some(cached) = cached {
            new = cached.update_entity(value);
        } else {
            new = value;
        }

        self.cache.cache_remove(&key)?;
        self.cache.cache_set(key, new)
    }

    pub fn get(&self, key: &K) -> Result<Option<V>, DiskCacheError> {
        self.cache.cache_get(key)
    }
}
