use crate::models::data::musicbrainz::url::mbid::URLMBID;
use derive_getters::Getters;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

pub mod caching;
pub mod converters;
pub mod mbid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Getters)]
pub struct URL {
    id: URLMBID,
    resource: String,
    tags: Option<Vec<Tag>>,
}
