use std::{fmt::Display, sync::Arc};

use cached::{DiskCache, DiskCacheError, IOCached};
use chashmap::CHashMap;
use color_eyre::eyre::Context;

use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::models::api::FetchAPI;

use super::{traits::merge::UpdateCachedEntity, CACHE_LOCATION};
use std::hash::Hash;

pub struct DiskCacheWrapper<K, V> {
    cache: DiskCache<K, V>,
    watch_cache: CHashMap<K, Arc<Semaphore>>,
}

impl<K, V> DiskCacheWrapper<K, V>
where
    K: Display + Eq + Hash + Clone,
    V: Serialize + DeserializeOwned + UpdateCachedEntity + FetchAPI<K, V>,
{
    pub fn new(name: &str) -> Self {
        Self {
            cache: DiskCache::new(name)
                .set_disk_directory(CACHE_LOCATION.clone())
                .build()
                .unwrap(),
            watch_cache: CHashMap::new(),
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

    fn get_semaphore(&self, key: &K) -> Arc<Semaphore> {
        if let Some(semaphore) = self.watch_cache.get(key) {
            return (*semaphore).clone();
        }

        self.watch_cache
            .insert(key.clone(), Arc::new(Semaphore::new(1)));
        return (*self
            .watch_cache
            .get(key)
            .expect("Couldn't get a new semaphore"))
        .clone();
    }

    pub async fn get_or_fetch(&self, key: &K) -> color_eyre::Result<V> {
        let semaphore = self.get_semaphore(key);
        let permit = semaphore.acquire().await.context("Couldn't get permit")?;

        let maybe_data = self.get(key)?;
        if let Some(data) = maybe_data {
            return Ok(data);
        }

        self.fetch(key, permit).await
    }

    /// Fetch an item, bypassing the cache. Only one request is allowed at a time, so a Semaphore permit is required. If none is provided, it will get assigned automatically.
    /// ⚠️ Waiting for a permit doesn't cancel the request. It only delays it. If the intention is to only fetch once, see [Self::get_or_fetch]
    pub async fn fetch<'a>(&self, key: &K, _permit: SemaphorePermit<'a>) -> color_eyre::Result<V> {
        V::fetch_and_insert(key).await
    }
}
