pub mod caching;
pub mod converter;
pub mod fetching;
pub mod mbid;
use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::tag::Tag;
use musicbrainz_rs::entity::work::WorkAttribute;
use musicbrainz_rs::entity::work::WorkType;
use serde::Deserialize;
use serde::Serialize;

use self::mbid::WorkMBID;

pub mod external;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Work {
    id: WorkMBID,
    title: String,
    type_id: Option<String>,
    work_type: Option<WorkType>,
    language: Option<String>,
    languages: Option<Vec<String>>,
    iswcs: Option<Vec<String>>,
    attributes: Option<Vec<WorkAttribute>>,
    disambiguation: Option<String>,
    //relations: Option<Vec<Relation>>,
    tags: Option<Vec<Tag>>,
    //rating: Option<Rating>,
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}
