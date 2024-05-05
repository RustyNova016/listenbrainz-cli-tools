use std::future::Future;

use super::insertable::Insertable;

pub trait InsertableWithChildren: Insertable + Sync {
    fn insert_with_children(
        &self,
        key: String,
    ) -> impl Future<Output = color_eyre::Result<()>> + Send {
        async { self.insert_into_cache_as(key).await }
    }
}

/// Wrapper to treat [InsertableWithChildren] as an [Insertable]
pub struct InsertChildren<T: InsertableWithChildren + Sync> {
    data: T,
}

impl<T: InsertableWithChildren + Sync> From<T> for InsertChildren<T> {
    fn from(value: T) -> Self {
        Self { data: value }
    }
}

impl<T: InsertableWithChildren + Sync> Insertable for InsertChildren<T> {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        self.data.insert_with_children(key).await
    }
}
