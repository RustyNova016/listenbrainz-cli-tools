use super::{cached::Cached, insertable::InsertableAs};
use std::fmt::Display;

/// Trait for all the entities that can be fetched
pub trait Fetchable<K> {
    /// Fetch the entity
    fn fetch(
        key: &K,
    ) -> impl std::future::Future<Output = color_eyre::Result<impl InsertableAs<K, Self>>> + Send
    where
        Self: Sized;
}

// --------
pub trait FetchableAndCachable<K: Display + Clone>: Fetchable<K> + Cached<K> {
    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_cached_or_fetch(key: &K) -> impl std::future::Future<Output = color_eyre::Result<Self>> {
        let key = key.clone();
        async move {
            match Self::get_cache().get(&key).await? {
                Some(val) => Ok(val),
                None => Self::get_cache().get_or_fetch(&key).await,
            }
        }
    }

    //async fn fetch_and_save_with_permit<'a>(
    //    &self,
    //    key: &K,
    //    _permit: &SemaphorePermit<'a>,
    //) -> color_eyre::Result<()> {
    //    let res = Self::fetch(&key).await?;
    //    let entities = res.to_entities();
    //
    //    //Self::get_cache().set(&key, entities.0).await?;
    //
    //    todo!()
    //    //for other_entity in entities.1 {
    //    //
    //    //}
    //}
}

impl<K, V> FetchableAndCachable<K> for V
where
    K: Display + Clone,
    V: Fetchable<K> + Cached<K>,
{
}
