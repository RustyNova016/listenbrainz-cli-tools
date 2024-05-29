use derive_getters::Getters;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::url::mbid::URLMBID;

pub mod converters;
pub mod mbid;
pub mod caching;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Getters)]
pub struct URL {
    id: URLMBID,
    resource: String,
    tags: Option<Vec<Tag>>,
}