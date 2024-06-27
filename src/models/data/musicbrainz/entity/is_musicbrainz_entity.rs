use std::sync::Arc;
use futures::Future;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveMBID;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::error::Error;

use super::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity_kind::MusicbrainzEntityKind;

pub trait IsMusicbrainzEntity
where
    Self: Clone + Serialize + DeserializeOwned,
{
    fn as_kind(&self) -> MusicbrainzEntityKind;

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID>;

    fn update(self, other: Self) -> Self;

    fn into_arc_and_any(self) -> AnyMusicBrainzEntity {
        Arc::new(self).into_any()
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity;

    fn get_cache() -> Arc<MusicbrainzCache<Self>>;

    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_cached_or_fetch(key: &NaiveMBID<Self>) -> impl Future<Output = color_eyre::Result<Arc<Self>>> {
        async move {
            Self::get_cache().get_load_or_fetch(key).await
        }
    }

    async fn save(self: Arc<Self>) -> color_eyre::Result<()> {
        Self::get_cache().update(self).await
    }

    #[must_use]
    async fn refresh(&self) -> color_eyre::Result<Arc<Self>> {
        Self::get_cache()
            .force_fetch_entity(&self.get_mbidspe().into_naive())
            .await
    }

    fn get_mb_cache() -> Arc<MusicbrainzCache<Self>>;
}
