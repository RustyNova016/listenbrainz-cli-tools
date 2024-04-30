use super::serde_cacache::SerdeCacache;
use crate::core::entity_traits::insertable::InsertableAs;
use crate::core::{caching::CACHE_LOCATION, entity_traits::fetchable::Fetchable};
use cacache::Integrity;
use chashmap::CHashMap;
use color_eyre::eyre::Context;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Display, sync::Arc};
use tokio::sync::{Semaphore, SemaphorePermit};

#[derive(Debug)]
pub struct EntityCache<K, V> {
    cache: SerdeCacache<K, V>,
    watch_cache: CHashMap<String, Arc<Semaphore>>,
}

impl<K, V> EntityCache<K, V>
where
    K: Display,
    V: Serialize + DeserializeOwned,
{
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);
        Self {
            cache: SerdeCacache::new(location),
            watch_cache: CHashMap::new(),
        }
    }

    pub async fn set(&self, key: &K, value: V) -> color_eyre::Result<Integrity> {
        self.cache.set(key, &value).await
    }

    pub async fn get(&self, key: &K) -> color_eyre::Result<Option<V>> {
        self.cache.get(key).await
    }

    fn get_semaphore(&self, key: &K) -> Arc<Semaphore> {
        if let Some(semaphore) = self.watch_cache.get(&key.to_string()) {
            return (*semaphore).clone();
        }

        self.watch_cache
            .insert(key.to_string(), Arc::new(Semaphore::new(1)));
        return (*self
            .watch_cache
            .get(&key.to_string())
            .expect("Couldn't get a new semaphore"))
        .clone();
    }
}

impl<K, V> EntityCache<K, V>
where
    K: Display + Clone,
    V: Serialize + DeserializeOwned + Fetchable<K>,
{
    /// Fetch an item, bypassing the cache. This also save the request.
    /// Only one request is allowed at a time, so a Semaphore permit is required.
    /// If none is provided, it will get assigned automatically.
    ///
    /// ⚠️ Waiting for a permit doesn't cancel the request. It only delays it.
    /// If the intention is to only fetch once, see [Self::get_or_fetch]
    pub async fn fetch_and_save(&self, key: K) -> color_eyre::Result<Option<V>> {
        let semaphore = self.get_semaphore(&key);
        let permit = semaphore.acquire().await.context("Couldn't get permit")?;

        self.fetch_and_save_with_permit(&key, &permit).await?;
        self.get(&key).await
    }

    async fn fetch_and_save_with_permit<'a>(
        &self,
        key: &K,
        _permit: &SemaphorePermit<'a>,
    ) -> color_eyre::Result<()> {
        V::fetch(key)
            .await?
            .insert_into_cache_as(key.clone())
            .await?;
        Ok(())
    }

    /// Get an element, and if it doesn't exist, fetch it
    pub async fn get_or_fetch(&self, key: &K) -> color_eyre::Result<V> {
        let semaphore = self.get_semaphore(key);
        let permit = semaphore.acquire().await.context("Couldn't get permit")?;

        let maybe_data = self.get(key).await?;
        if let Some(data) = maybe_data {
            return Ok(data);
        }

        self.fetch_and_save_with_permit(key, &permit).await?;
        Ok(self
            .get(key)
            .await?
            .expect("Entity couldn't be found after insertion"))
    }
}
