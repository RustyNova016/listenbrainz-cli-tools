use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use chrono::NaiveDate;
use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::release_group::{ReleaseGroupPrimaryType, ReleaseGroupSecondaryType};
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

mod caching;
mod converters;
pub(crate) mod external;
mod fetching;
pub mod mbid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct ReleaseGroup {
    id: ReleaseGroupMBID,
    primary_type_id: Option<String>,
    primary_type: Option<ReleaseGroupPrimaryType>,
    secondary_type_ids: Vec<String>,
    secondary_types: Vec<ReleaseGroupSecondaryType>,
    first_release_date: Option<NaiveDate>,
    title: String,
    disambiguation: String,
    relations: Option<Vec<Relation>>,
    artist_credit: Option<ArtistCredits>,
    releases: Option<Vec<ReleaseMBID>>,
    tags: Option<Vec<Tag>>,
    //rating: Option<Rating>,
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl HasArtistCredits for ReleaseGroup {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}
