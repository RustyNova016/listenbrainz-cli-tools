use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveMBID;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::error::Error;

use super::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity_kind::MusicbrainzEntityKind;

#[deprecated]
pub trait IsMusicbrainzEntity
where
    Self: Clone + Serialize + DeserializeOwned + Eq,
    NaiveMBID<Self>: IsMusicbrainzID<Self>,
{
    fn as_kind(&self) -> MusicbrainzEntityKind;

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID>;

    #[must_use]
    fn partial_update(self, other: Self) -> Self;

    fn into_arc_and_any(self) -> AnyMusicBrainzEntity {
        Arc::new(self).into_any()
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity;

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error>;
}
