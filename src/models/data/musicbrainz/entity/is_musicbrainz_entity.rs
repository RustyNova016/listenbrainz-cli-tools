use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};

use super::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity_kind::MusicbrainzEntityKind;

pub trait IsMusicbrainzEntity
where
    Self: Clone + Serialize + DeserializeOwned,
{
    fn as_kind(&self) -> MusicbrainzEntityKind;

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID>;

    fn update(self, other: Self) -> Self;

    fn into_any(self) -> AnyMusicBrainzEntity;
}
