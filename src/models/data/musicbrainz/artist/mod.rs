pub mod external;
use serde::{Deserialize, Serialize};

pub mod caching;
pub mod fetching;

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
