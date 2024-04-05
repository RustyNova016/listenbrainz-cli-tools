
use serde::{Deserialize, Serialize};

use crate::models::musicbrainz::artist_credit::collection::ArtistCredits;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub artist_credit: Option<ArtistCredits>,
}

impl From<musicbrainz_rs::entity::recording::Recording> for Recording {
    fn from(recording: musicbrainz_rs::entity::recording::Recording) -> Self {
        Self {
            id: recording.id,
            title: recording.title,
            artist_credit: recording
                .artist_credit
                .map(|coll| coll.into()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Artist {
    pub id: String,
    pub name: String,
}

impl From<musicbrainz_rs::entity::artist::Artist> for Artist {
    fn from(artist: musicbrainz_rs::entity::artist::Artist) -> Self {
        Self {
            id: artist.id,
            name: artist.name,
        }
    }
}
