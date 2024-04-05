use serde::{Deserialize, Serialize};

use crate::models::data::recording::Artist;

pub mod caching;
pub mod collection;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: Option<String>,
    pub artist: Artist,
}

impl From<musicbrainz_rs::entity::artist_credit::ArtistCredit> for ArtistCredit {
    fn from(artist_credit: musicbrainz_rs::entity::artist_credit::ArtistCredit) -> Self {
        Self {
            name: artist_credit.name,
            joinphrase: artist_credit.joinphrase,
            artist: artist_credit.artist.into(),
        }
    }
}