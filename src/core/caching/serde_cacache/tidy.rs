use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use cacache::{Integrity, Metadata};
use chashmap::CHashMap;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::core::caching::serde_cacache::error::Error;

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
        let key = key.to_string();
        let serialized = rmp_serde::to_vec(&(key.clone(), data))?;

        self.inner_delete_entry(&key).await?;
        Ok(self.inner_write(&key, &serialized).await?)
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
        let key = key.to_string();
        match self.inner_read(&key).await {
            Ok(val) => {
                let content: (String, V) = rmp_serde::from_slice(&val)?;
                Ok(content.1)
            }

            Err(cacache::Error::EntryNotFound(a, b)) => Err(Error::EntryNotFound(a, b)),

            Err(val) => Err(Error::CacheError(val)),
        }
    }

    pub async fn remove(&self, key: &K) -> Result<(), cacache::Error> {
        self.inner_delete_entry(&key.to_string()).await
    }

    #[must_use]
    fn get_lock(&self, key: &String) -> Arc<RwLock<String>> {
        match self.value_locks.get(key) {
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
    async fn inner_write(&self, key: &String, data: &[u8]) -> Result<Integrity, cacache::Error> {
        let lock = self.get_lock(key);
        let _check_lock = self.cache_lock.read().await;
        let _write_lock = lock.write().await;

        cacache::write(&self.name, key.to_string(), data).await
    }

    /// Access to [`cacache::read`] with locking behavior
    async fn inner_read(&self, key: &String) -> Result<Vec<u8>, cacache::Error> {
        let lock = self.get_lock(key);
        let _check_lock = self.cache_lock.read().await;
        let _read_lock = lock.read().await;

        cacache::read(&self.name, key.to_string()).await
    }

    async fn inner_delete_entry(&self, key: &String) -> Result<(), cacache::Error> {
        let lock = self.get_lock(key);
        let _activate_lock = self.cache_lock.write().await;
        let _write_lock = lock.write().await;

        let Some(metadata_of_entry) = cacache::metadata(&self.name, key.to_string()).await? else {
            return Ok(());
        };

        // All entries have been entered with their key, making all the duplicates uniques.
        // So it's safe* to just delete the hash
        //
        // Hash collisions may still occure, but if we worry about those, we should worry about using cacache in the first place
        cacache::remove(&self.name, key.to_string()).await?;
        cacache::remove_hash(&self.name, &metadata_of_entry.integrity).await?;

        Ok(())
    }

    pub async fn delete_last_entries(
        &self,
        k: usize,
        keep_min: usize,
    ) -> Result<(), cacache::Error> {
        let entries_result = cacache::list_sync(&self.name).collect::<Result<Vec<Metadata>, _>>();

        let entries;
        match entries_result {
            // Got data? Good!
            Ok(val) => entries = val,

            // If we cannot find the directory, it's fine. We just didn't initialize it yet
            Err(cacache::Error::IoError(val, comp)) => {
                if val.kind() == std::io::ErrorKind::NotFound {
                    return Ok(());
                } else {
                    // Wasn't a missing directory? Then that's bad
                    return Err(cacache::Error::IoError(val, comp));
                }
            }

            // ... But in other cases this is bad! Send it back up.
            Err(val) => return Err(val),
        }

        // Keep a minimum of `keep_min` entries.
        if entries.is_empty() || entries.len() <= keep_min {
            return Ok(());
        }

        let entries_to_delete = entries
            .into_iter()
            .k_smallest_by_key(k, |entry| entry.time)
            .collect_vec();

        for entry_to_delete in entries_to_delete {
            self.inner_delete_entry(&entry_to_delete.key).await?;
        }

        Ok(())
    }
}
