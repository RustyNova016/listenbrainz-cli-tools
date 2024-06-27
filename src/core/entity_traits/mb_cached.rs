use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

pub trait MBCached<K>
where
    K: IsMbid<Self> + Serialize + DeserializeOwned,
    Self: Serialize + DeserializeOwned + HasMBID<K> + IsMusicbrainzEntity + Clone,
{
    fn get_cache() -> Arc<MusicbrainzCacheLegacy<K, Self>>;

    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_cached_or_fetch(key: &K) -> impl std::future::Future<Output = color_eyre::Result<Self>> {
        async move {
            match Self::get_cache().get(key).await? {
                Some(val) => Ok(val),
                None => Self::get_cache().get_or_fetch(key).await,
            }
        }
    }

    async fn save(&self) -> color_eyre::Result<()> {
        Self::get_cache().update(self).await
    }

    async fn refresh(&self) -> color_eyre::Result<Self> {
        Self::get_cache()
            .force_fetch_and_save(&self.get_mbid())
            .await
    }
}
