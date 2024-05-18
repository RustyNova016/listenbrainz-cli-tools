pub mod caching;
pub mod converter;
pub mod fetching;
pub mod getters;
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

use super::relation::Relation;

pub mod external;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Getters)]
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
    relations: Option<Vec<Relation>>,
    tags: Option<Vec<Tag>>,
    //rating: Option<Rating>, TODO: Create own struct that have Eq
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl Work {
    pub fn create_fake_work(id: String, title: String) -> Self {
        Self {
            id: id.into(),
            title,
            languages: Some(Vec::new()),
            iswcs: Some(Vec::new()),
            attributes: Some(Vec::new()),
            relations: Some(Vec::new()),
            tags: Some(Vec::new()),
            aliases: Some(Vec::new()),
            genres: Some(Vec::new()),
            annotation: Some(String::new()),
            disambiguation: Some(String::new()),
            language: Some(String::new()),
            type_id: Some(String::new()),
            work_type: None,
        }
    }
}
