pub mod error;
use std::fmt::Display;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::core::caching::serde_cacache::error::Error;
use cacache::Integrity;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct SerdeCacache<K, V> {
    name: PathBuf,
    _phantom_data: PhantomData<V>,
    _phantom_key: PhantomData<K>,
}

impl<K, V> SerdeCacache<K, V>
where
    V: Serialize + DeserializeOwned,
    K: Display,
{
    pub fn new(name: PathBuf) -> Self {
        Self {
            name,
            _phantom_data: PhantomData,
            _phantom_key: PhantomData,
        }
    }

    /// Set an item in the cache
    pub async fn set(&self, key: &K, data: &V) -> Result<Integrity, Error> {
        let serialized = rmp_serde::to_vec(data)?;
        Ok(cacache::write(&self.name, key.to_string(), serialized).await?)
    }

    /// Get an item and return an option if it isn't found. This is more akin to a [`HashMap`](std::collections::HashMap)
    pub async fn get(&self, key: &K) -> Result<Option<V>, Error> {
        let read = cacache::read(&self.name, key.to_string()).await;

        match read {
            Ok(val) => Ok(Some(rmp_serde::from_slice(&val)?)),
            Err(cacache::Error::EntryNotFound(_, _)) => Ok(None),
            val => {
                val?;
                Ok(None)
            }
        }
    }

    /// Get an item from the cache.
    pub async fn get_as_result(&self, key: &K) -> Result<V, Error> {
        let read: Vec<u8> = cacache::read(&self.name, key.to_string()).await?;
        Ok(rmp_serde::from_slice(&read)?)
    }
}
