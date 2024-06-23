use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::artist::{ArtistType, Gender};
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::lifespan::LifeSpan;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;

use super::recording::mbid::RecordingMBID;
use super::release::mbid::ReleaseMBID;
use super::release_group::mbid::ReleaseGroupMBID;

use self::mbid::ArtistMBID;

pub mod caching;
pub mod external;
pub mod getters;
pub mod mbid;
mod relations;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct Artist {
    id: ArtistMBID,
    name: String,
    sort_name: String,
    disambiguation: String,
    artist_type: Option<ArtistType>,
    gender: Option<Gender>,
    //pub area: Option<Area>,
    //pub begin_area: Option<Area>,
    relations: Option<Vec<Relation>>,
    releases: Option<Vec<ReleaseMBID>>,
    works: Option<Vec<WorkMBID>>,
    release_groups: Option<Vec<ReleaseGroupMBID>>,
    recordings: Option<Vec<RecordingMBID>>,
    aliases: Option<Vec<Alias>>,
    tags: Option<Vec<Tag>>,
    genres: Option<Vec<Genre>>,
    //pub rating: Option<Rating>,
    country: Option<String>,
    annotation: Option<String>,
    life_span: Option<LifeSpan>,
}

impl From<musicbrainz_rs::entity::artist::Artist> for Artist {
    fn from(artist: musicbrainz_rs::entity::artist::Artist) -> Self {
        Self {
            id: artist.id.into(),
            name: artist.name,
            aliases: artist.aliases,
            annotation: artist.annotation,
            artist_type: artist.artist_type,
            country: artist.country,
            disambiguation: artist.disambiguation,
            gender: artist.gender,
            genres: artist.genres,
            life_span: artist.life_span,
            recordings: None,
            release_groups: None,
            releases: None,
            sort_name: artist.sort_name,
            tags: artist.tags,
            works: None,
            relations: artist
                .relations
                .map(|relations| relations.into_iter().map_into().collect_vec()),
        }
    }
}
