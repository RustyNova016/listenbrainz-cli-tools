pub mod converters;
use std::collections::HashMap;

use chrono::NaiveDate;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use super::artist::mbid::ArtistMBID;
use super::recording::mbid::RecordingMBID;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Relation {
    end: Option<NaiveDate>,
    attributes: Option<Vec<String>>,
    content: RelationTarget,
    attribute_values: Option<HashMap<String, String>>,
    attribute_ids: Option<HashMap<String, String>>,
    target_type: Option<String>,
    target_credit: Option<String>,
    source_credit: Option<String>,
    ended: Option<bool>,
    type_id: String,
    begin: Option<NaiveDate>,
    direction: String,
    relation_type: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum RelationTarget {
    Artist(ArtistMBID),
    Recording(RecordingMBID),

    //TODO: Remove at 1.0!
    Unknown(),
}
