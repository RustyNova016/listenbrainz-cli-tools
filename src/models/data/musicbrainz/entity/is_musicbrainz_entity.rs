use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};

use super::entity_kind::MusicbrainzEntityKind;

pub trait IsMusicbrainzEntity
where
    Self: Clone + Serialize + DeserializeOwned,
{
    fn as_kind(&self) -> MusicbrainzEntityKind;

    fn get_mbid(&self) -> MBIDSpe<Self, PrimaryID>;
}
