use crate::core::entity_traits::mb_cached::MBCached;

use super::HasMBID;
use super::IsMbid;

pub trait IsCachedMBID<T>
where
    Self: IsMbid<T>,
    T: HasMBID<Self> + MBCached<Self>,
{
    async fn get_primary_alias(&self) -> color_eyre::Result<Self> {
        Ok(T::get_cache().get_primary_mbid_alias(self).await?)
    }

    async fn get_or_fetch_primary_mbid_alias(&self) -> color_eyre::Result<Self> {
        T::get_cache().get_or_fetch_primary_mbid_alias(self).await
    }
}

impl<K, T> IsCachedMBID<T> for K
where
    Self: IsMbid<T>,
    T: HasMBID<Self> + MBCached<Self>,
{
}

