use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

pub mod caching;
pub mod collection;
pub mod converters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct ArtistCredit {
    name: String,
    joinphrase: Option<String>,
    artist: ArtistMBID,
}
