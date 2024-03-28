use musicbrainz_rs::{Fetch, FetchQuery};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

impl Recording {
    pub fn fetch() -> FetchQuery<musicbrainz_rs::entity::recording::Recording> {
        musicbrainz_rs::entity::recording::Recording::fetch()
    }
}

impl From<musicbrainz_rs::entity::recording::Recording> for Recording {
    fn from(recording: musicbrainz_rs::entity::recording::Recording) -> Self {
        Self {
            id: recording.id,
            title: recording.title,
            artist_credit: recording
                .artist_credit
                .map(|coll| coll.into_iter().map(|c| c.into()).collect()),
        }
    }
}

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
