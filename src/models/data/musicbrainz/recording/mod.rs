pub mod convertion;
pub mod external;
pub mod get_or_fetch;
pub mod mbid;

use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use self::mbid::RecordingMBID;

use super::artist_credit::collection::ArtistCredits;
use super::relation::Relation;

pub mod caching;
pub mod fetching;

// impl HasMbid for Recording {
//     fn get_mbid(&self) -> &str {
//         &self.id
//     }
// }

impl Recording {
    pub fn get_artist_credits(&self) -> Option<ArtistCredits> {
        self.artist_credit.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Recording {
    pub id: RecordingMBID,
    pub title: String,
    pub artist_credit: Option<ArtistCredits>,
    releases: Option<Vec<ReleaseMBID>>,
    video: Option<bool>,
    length: Option<u32>,
    disambiguation: Option<String>,
    isrcs: Option<Vec<String>>,
    relations: Option<Vec<Relation>>,
    aliases: Option<Vec<Alias>>,
    tags: Option<Vec<Tag>>,
    //rating: Option<Rating>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl From<musicbrainz_rs::entity::recording::Recording> for Recording {
    fn from(recording: musicbrainz_rs::entity::recording::Recording) -> Self {
        Self {
            id: recording.id.into(),
            title: recording.title,
            artist_credit: recording.artist_credit.map(|coll| coll.into()),
            releases: recording.releases.map(|releases| {
                releases
                    .into_iter()
                    .map(|release| release.id.into())
                    .collect_vec()
            }),
            length: recording.length,
            video: recording.video,
            aliases: recording.aliases,
            genres: recording.genres,
            annotation: recording.annotation,
            tags: recording.tags,
            isrcs: recording.isrcs,
            disambiguation: recording.disambiguation,
            relations: recording
                .relations
                .map(|relations| relations.into_iter().map_into().collect_vec()),
        }
    }
}
