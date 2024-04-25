use crate::core::entity_traits::fetch_api::FetchAPI;
use crate::core::entity_traits::has_cache::HasCache;
use crate::core::entity_traits::merge::UpdateCachedEntity;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;
use std::future::Future;
use std::hash::Hash;

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
