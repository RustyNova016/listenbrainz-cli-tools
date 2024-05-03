use crate::core::caching::entity_cache::EntityCache;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// For all the entities that have a cache
pub trait Cached: Serialize + DeserializeOwned  + Clone + PartialEq + Eq {
    /// Get the cache correponding to the entity
    fn get_cache() -> Arc<EntityCache<Self>>
    where
        Self: Sized;
}

// pub trait CachedExtra<K: Clone + Display>: Cached<K> + Fetchable<K> {
//     //async fn set(key: K, value: Self) -> color_eyre::Result<Integrity> where Self: Sized + Serialize + DeserializeOwned, K: Display{
//     //    Self::get_cache().set(key, value).await
//     //}
// //
//     //async fn set_or_update(key: K, value: Self) -> impl Future<Output = color_eyre::Result<()>> where Self: Sized + Serialize + DeserializeOwned, K: Display{
//     //    Self::get_cache().set_or_update(key, value).await
//     //}

//     fn get_or_fetch(key: K) -> impl Future<Output = Result<Self, color_eyre::eyre::Error>> {
//         Self::get_cache().get_or_fetch(&key)
//     }
// }
