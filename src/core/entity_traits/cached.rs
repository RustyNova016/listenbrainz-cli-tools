use crate::core::caching::entity_cache::EntityCache;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

#[deprecated]
/// For all the entities that have a cache
pub trait Cached: Serialize + DeserializeOwned + Clone + PartialEq + Eq {
    #[deprecated]
    /// Get the cache correponding to the entity
    fn get_cache() -> Arc<EntityCache<Self>>
    where
        Self: Sized;
}