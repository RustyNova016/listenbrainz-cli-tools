use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use std::{fmt::Display, future::Future};

use super::cache::traits::has_cache::HasCache;
use super::cache::traits::merge::UpdateCachedEntity;

pub trait FetchAPI<K, V> {
    /// Fetch an item an put it into the cache
    ///
    /// This operation isn't deduplicated! Refer to the Diskcache for safe call
    fn fetch_and_insert(key: &K) -> impl Future<Output = color_eyre::Result<V>>;
}

pub trait GetFromCacheOrFetch<K, V>: HasCache<K, V> + FetchAPI<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: DeserializeOwned + Serialize + UpdateCachedEntity + FetchAPI<K, V>,
{
    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_cached_or_fetch(key: &K) -> impl Future<Output = color_eyre::Result<V>> {
        async {
            match Self::get_cache().get(key)? {
                Some(val) => Ok(val),
                None => Self::get_cache().get_or_fetch(key).await,
            }
        }
    }
}

impl<K, V, T> GetFromCacheOrFetch<K, V> for T
where
    K: Eq + Hash + Clone + Display,
    V: DeserializeOwned + Serialize + UpdateCachedEntity + FetchAPI<K, V>,
    T: HasCache<K, V> + FetchAPI<K, V>,
{
}
