use super::has_id::HasID;
use std::{fmt::Display, future::Future};

pub trait InsertableAs<V> {
    // Insert everything into the global cache
    fn insert_into_cache_as(&self, key: String) -> impl Future<Output = color_eyre::Result<()>> + Send;
}

pub trait IsAutoInsertableAs<V>: HasID + InsertableAs<V> {
    fn insert_into_cache<'a>(&'a self) -> impl Future<Output = color_eyre::Result<()>> + Send
    {
        self.insert_into_cache_as(self.get_id())
    }

    /// Insert the value int
    fn insert_into_cache_along_key(&self, key: String) -> impl Future<Output = color_eyre::Result<()>> {
        async {
            self.insert_into_cache_as(self.get_id()).await?;
            self.insert_into_cache_as(key).await?;
            Ok(())
        }
    }
}

impl<V, T: HasID + InsertableAs<V>> IsAutoInsertableAs<V> for T {}

pub trait InsertableWithExtras<V>: InsertableAs<V> + HasID {
    fn insert_with_relations(&self, key: String) -> impl Future<Output = color_eyre::Result<()>> + Send;
}
