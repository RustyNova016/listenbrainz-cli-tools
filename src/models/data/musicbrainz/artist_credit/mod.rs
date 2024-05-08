use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;

pub mod caching;
pub mod collection;
pub mod converters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: Option<String>,
    pub artist: ArtistMBID,
}
