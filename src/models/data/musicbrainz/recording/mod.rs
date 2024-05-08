pub mod convertion;
pub mod external;
pub mod getters;
pub mod id;
use crate::models::data::entity_database::ENTITY_DATABASE;

use color_eyre::eyre::{eyre, Context, OptionExt};
use color_eyre::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::artist_credit::collection::ArtistCredits;
use super::HasMbid;

pub mod caching;
pub mod fetching;

impl HasMbid for Recording {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

impl Recording {
    pub fn get_artist_credits(&self) -> Option<ArtistCredits> {
        self.artist_credit.clone()
    }


}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub artist_credit: Option<ArtistCredits>,
    releases: Option<Vec<String>>,
}

impl From<musicbrainz_rs::entity::recording::Recording> for Recording {
    fn from(recording: musicbrainz_rs::entity::recording::Recording) -> Self {
        Self {
            id: recording.id,
            title: recording.title,
            artist_credit: recording.artist_credit.map(|coll| coll.into()),
            releases: recording
                .releases
                .map(|releases| releases.into_iter().map(|release| release.id).collect_vec()),
        }
    }
}
