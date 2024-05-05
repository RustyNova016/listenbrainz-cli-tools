use std::sync::Arc;

use chashmap::CHashMap;
use color_eyre::eyre::Context;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::core::entity_traits::insertable::Insertable;
use crate::core::entity_traits::updatable::Updatable;
use crate::core::{caching::CACHE_LOCATION, entity_traits::fetchable::Fetchable};

use super::serde_cacache::SerdeCacache;

#[derive(Debug)]
pub struct EntityCache<V> {
    cache: SerdeCacache<String, V>,
    watch_cache: CHashMap<String, Arc<Semaphore>>,
}

impl<V> EntityCache<V>
where
    V: Serialize + DeserializeOwned
{
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);
        Self {
            cache: SerdeCacache::new(location),
            watch_cache: CHashMap::new(),
        }
    }

    pub async fn set(&self, key: &String, value: V) -> color_eyre::Result<()> {
        self.cache.set(key, &value).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> color_eyre::Result<Option<V>> {
        self.cache.get(&key.to_string()).await
    }

    fn get_semaphore(&self, key: &str) -> Arc<Semaphore> {
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

impl<V> EntityCache<V>
where
    V: Serialize + DeserializeOwned + Fetchable  + Clone + PartialEq + Eq,
{
    /// Fetch an item, bypassing the cache. This also save the request.
    /// Only one request is allowed at a time, so a Semaphore permit is required.
    /// If none is provided, it will get assigned automatically.
    ///
    /// ⚠️ Waiting for a permit doesn't cancel the request. It only delays it.
    /// If the intention is to only fetch once, see [Self::get_or_fetch]
    pub async fn fetch_and_save(&self, key: String) -> color_eyre::Result<Option<V>> {
        let semaphore = self.get_semaphore(&key);
        let permit = semaphore.acquire().await.context("Couldn't get permit")?;

        self.fetch_and_save_with_permit(&key, &permit).await?;
        self.get(&key).await
    }

    async fn fetch_and_save_with_permit<'a>(
        &self,
        key: &str,
        _permit: &SemaphorePermit<'a>,
    ) -> color_eyre::Result<()> {
        V::fetch(key)
            .await?
            .insert_into_cache_as(key.to_string())
            .await?;
        Ok(())
    }

    /// Get an element, and if it doesn't exist, fetch it
    pub async fn get_or_fetch(&self, key: &str) -> color_eyre::Result<V> {
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

impl<V> EntityCache<V>
where
    V: Serialize + DeserializeOwned + Updatable,
{
    pub async fn update(&self, key: &String, value: V) -> color_eyre::Result<()> {
        let older = self.get(key).await?;

        if let Some(older) = older {
            self.set(key, older.update(value)).await?;
        } else {
            self.set(key, value).await?;
        }

        Ok(())
    }
}
