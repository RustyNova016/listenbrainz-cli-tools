use serde::de::DeserializeOwned;
use serde::Serialize;
use std::hash::Hash;
use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::core::entity_traits::updatable::Updatable;

pub trait MBCached<K>
where
    K: IsMbid<Self> + Serialize + DeserializeOwned + Eq + Hash,
    Self: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    fn get_cache() -> Arc<MusicbrainzCache<K, Self>>;

    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_cached_or_fetch(
        key: &K,
    ) -> impl std::future::Future<Output = color_eyre::Result<Arc<Self>>> {
        async move { Self::get_cache().get_or_fetched(key).await }
    }

    async fn save(&self) -> color_eyre::Result<()> {
        Self::get_cache().update(self).await
    }

    async fn refresh(&self) -> color_eyre::Result<Arc<Self>> {
        Self::get_cache().force_fetch_entity(&self.get_mbid()).await
    }
}
