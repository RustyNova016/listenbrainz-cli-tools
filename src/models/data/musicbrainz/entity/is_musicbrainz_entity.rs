use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};

use super::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity_kind::MusicbrainzEntityKind;

pub trait IsMusicbrainzEntity
where
    Self: Clone + Serialize + DeserializeOwned + Eq,
{
    fn as_kind(&self) -> MusicbrainzEntityKind;

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID>;

    fn update(self, other: Self) -> Self;

    fn into_arc_and_any(self) -> AnyMusicBrainzEntity {
        Arc::new(self).into_any()
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity;

    fn get_mb_cache() -> Arc<MusicbrainzCache<Self>>;
}
