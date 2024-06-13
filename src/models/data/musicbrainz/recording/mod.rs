pub mod getters;
use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;

use super::artist_credit::collection::ArtistCredits;
use super::relation::Relation;

use self::mbid::RecordingMBID;

pub mod caching;
pub mod converters;
pub mod external;
pub mod get_or_fetch;
pub mod mbid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Recording {
    id: RecordingMBID,
    title: String,
    artist_credit: Option<ArtistCredits>,
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

impl Recording {
    pub async fn get_title_with_credits(&self) -> color_eyre::Result<String> {
        let credit = self
            .get_or_fetch_artist_credits()
            .await?
            .get_artist_credit_as_string(); //.unwrap_or_else(|| "[unknown]".to_string());
        Ok(format!("{} by {}", self.title, credit))
    }
}
