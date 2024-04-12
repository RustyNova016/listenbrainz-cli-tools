use std::{fmt::Display, hash::Hash, sync::Arc};

use cached::DiskCacheError;
use serde::{de::DeserializeOwned, Serialize};

use crate::models::{api::FetchAPI, cache::disk_cache::DiskCacheWrapper};

use super::merge::UpdateCachedEntity;
pub trait HasCache<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: DeserializeOwned + Serialize + UpdateCachedEntity + FetchAPI<K, V>,
{
    fn get_cache() -> Arc<DiskCacheWrapper<K, V>>;

    fn get_from_cache(key: &K) -> Result<Option<V>, DiskCacheError> {
        Self::get_cache().get(key)
    }

    fn set(key: K, value: V) -> Result<Option<V>, DiskCacheError> {
        Self::get_cache().set(key, value)
    }

    fn set_or_update(key: K, value: V) -> Result<Option<V>, DiskCacheError> {
        Self::get_cache().set_or_update(key, value)
    }
}
