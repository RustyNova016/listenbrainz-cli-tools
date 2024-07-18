pub mod any_to_kind;
pub mod marker;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::error::Error;

use super::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity_kind::MusicbrainzEntityKind;

/// Marker trait for an `MusicBrainz` entity
pub trait MusicBrainzEntity: Serialize + DeserializeOwned + Eq + Clone {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity>;
    // --- Type methods ---

    /// Return the kind of this entity
    fn get_kind() -> MusicbrainzEntityKind;

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity;

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error>;

    fn into_arc_and_any(self) -> AnyMusicBrainzEntity {
        Arc::new(self).into_any()
    }

    // --- Cache methods ---

    fn get_cache() -> Arc<MusicbrainzCache<Self>>;

    /// Get the data from the cache, or call the API. Any request is deduplicated
    fn get_load_or_fetch(
        mbid: &NaiveMBID<Self>,
    ) -> impl std::future::Future<Output = color_eyre::Result<Arc<Self>>> {
        async move { Self::get_cache().get_load_or_fetch(mbid).await }
    }

    /// Update fields of the entity base on the available info of the other
    fn incremental_update(self, newer: Self) -> Self;

    // --- Data Methods ---

    /// Return the MBID of the entity
    fn get_mbid(&self) -> PrimaryMBID<Self>;
}
