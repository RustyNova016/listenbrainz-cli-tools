use std::{fmt::Display, future::Future};
use super::has_id::HasID;

pub trait InsertableAs<K, V> {
    // Insert everything into the global cache
    fn insert_into_cache_as(&self, key: K) -> impl Future<Output = color_eyre::Result<()>> + Send;
}

pub trait IsAutoInsertableAs<K: Display, V>: HasID<K> + InsertableAs<K, V> {
    fn insert_into_cache<'a>(&'a self) -> impl Future<Output = color_eyre::Result<()>> + Send
    where
        K: 'a,
    {
        self.insert_into_cache_as(self.get_id())
    }

    /// Insert the value int
    fn insert_into_cache_along_key(&self, key: K) -> impl Future<Output = color_eyre::Result<()>> {
        async {
            self.insert_into_cache_as(self.get_id()).await?;
            self.insert_into_cache_as(key).await
        }
    }
}

impl<K: Display, V, T: HasID<K> + InsertableAs<K, V>> IsAutoInsertableAs<K, V> for T {}

pub trait InsertableWithExtras<K: Display + Clone, V>: InsertableAs<K, V> + HasID<K> {
    fn insert_with_relations(&self, key: K) -> impl Future<Output = color_eyre::Result<()>> + Send;
}