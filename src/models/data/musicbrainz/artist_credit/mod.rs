use serde::{Deserialize, Serialize};

pub mod caching;
pub mod collection;
pub mod converters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: Option<String>,
    pub artist: String,
}
