use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use crate::core::caching::serde_cacache::error::Error;
use cacache::Integrity;
use chashmap::CHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct SerdeCacacheTidy<K, V> {
    name: PathBuf,
    _phantom_data: PhantomData<V>,
    _phantom_key: PhantomData<K>,

    cache_lock: Arc<RwLock<bool>>,
    value_locks: Arc<CHashMap<String, Arc<RwLock<String>>>>,
}

impl<K, V> SerdeCacacheTidy<K, V>
where
    V: Serialize + DeserializeOwned,
    K: Display,
{
    pub fn new(name: PathBuf) -> Self {
        Self {
            name,
            _phantom_data: PhantomData,
            _phantom_key: PhantomData,

            cache_lock: Arc::new(RwLock::new(false)),
            value_locks: Arc::new(CHashMap::new()),
        }
    }

    /// Set an item in the cache
    pub async fn set(&self, key: &K, data: &V) -> Result<Integrity, Error> {
        let serialized = rmp_serde::to_vec(&(key.to_string(), data))?;

        self.inner_delete_entry(key).await?;
        Ok(self.inner_write(key, &serialized).await?)
    }

    /// Get an item and return an option if it isn't found. This is more akin to a [`HashMap`](std::collections::HashMap)
    pub async fn get_or_option(&self, key: &K) -> Result<Option<V>, Error> {
        match self.get(key).await {
            Ok(val) => Ok(Some(val)),
            Err(Error::EntryNotFound(_, _)) => Ok(None),
            Err(val) => Err(val),
        }
    }

    /// Get an item from the cache.
    pub async fn get(&self, key: &K) -> Result<V, Error> {
        match self.inner_read(key).await {
            Ok(val) => {
                let content: (String, V) = rmp_serde::from_slice(&val)?;
                Ok(content.1)
            }

            Err(cacache::Error::EntryNotFound(a, b)) => Err(Error::EntryNotFound(a, b)),

            Err(val) => Err(Error::CacheError(val)),
        }
    }

    pub async fn remove(&self, key: &K) -> Result<(), cacache::Error> {
        self.inner_delete_entry(key).await
    }

    #[must_use]
    fn get_lock(&self, key: &K) -> Arc<RwLock<String>> {
        match self.value_locks.get(&key.to_string()) {
            Some(val) => val.deref().clone(),

            None => {
                self.value_locks
                    .insert(key.to_string(), Arc::new(RwLock::new(key.to_string())));

                self.value_locks
                    .get(&key.to_string())
                    .expect("Couldn't get just inserted lock")
                    .deref()
                    .clone()
            }
        }
    }

    /// Access to [`cacache::write`] with locking behavior
    async fn inner_write(&self, key: &K, data: &[u8]) -> Result<Integrity, cacache::Error> {
        let lock = self.get_lock(key);
        let _check_lock = self.cache_lock.read().await;
        let _write_lock = lock.write().await;

        cacache::write(&self.name, key.to_string(), data).await
    }

    /// Access to [`cacache::read`] with locking behavior
    async fn inner_read(&self, key: &K) -> Result<Vec<u8>, cacache::Error> {
        let lock = self.get_lock(key);
        let _check_lock = self.cache_lock.read().await;
        let _read_lock = lock.read().await;

        cacache::read(&self.name, key.to_string()).await
    }

    async fn inner_delete_entry(&self, key: &K) -> Result<(), cacache::Error> {
        let lock = self.get_lock(key);
        let _activate_lock = self.cache_lock.write().await;
        let _write_lock = lock.write().await;

        let Some(metadata_of_entry) = cacache::metadata(&self.name, key.to_string()).await? else {
            return Ok(());
        };

        let indexes = cacache::list_sync(&self.name);

        // We check if the cache content is reused somewhere. If it isn't, it is safe to clean
        let mut content_usage_count = 0;
        for index in indexes {
            if index?.integrity == metadata_of_entry.integrity {
                content_usage_count += 1;
            }

            if content_usage_count > 1 {
                break;
            }
        }

        cacache::remove(&self.name, key.to_string()).await?;

        if content_usage_count < 2 {
            cacache::remove_hash(&self.name, &metadata_of_entry.integrity).await?;
        }

        Ok(())
    }
}
