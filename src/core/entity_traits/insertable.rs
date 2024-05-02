use super::has_id::HasID;
use std::future::Future;

pub trait Insertable {
    // Insert everything into the global cache
    fn insert_into_cache_as(
        &self,
        key: String,
    ) -> impl Future<Output = color_eyre::Result<()>> + Send;
}

pub trait IsAutoInsertable: HasID + Insertable {
    fn insert_into_cache(&self) -> impl Future<Output = color_eyre::Result<()>> + Send {
        self.insert_into_cache_as(self.get_id())
    }

    /// Insert the value int
    fn insert_into_cache_along_key(
        &self,
        key: String,
    ) -> impl Future<Output = color_eyre::Result<()>> {
        async {
            self.insert_into_cache_as(self.get_id()).await?;
            self.insert_into_cache_as(key).await?;
            Ok(())
        }
    }
}

impl<T: HasID + Insertable> IsAutoInsertable for T {}

pub trait InsertableWithExtras<V>: Insertable + HasID {
    fn insert_with_relations(
        &self,
        key: String,
    ) -> impl Future<Output = color_eyre::Result<()>> + Send;
}
