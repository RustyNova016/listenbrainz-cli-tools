pub mod converters;
use derive_more::IsVariant;
use derive_more::Unwrap;
use std::collections::HashMap;

use chrono::NaiveDate;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use super::artist::mbid::ArtistMBID;
use super::recording::mbid::RecordingMBID;
use super::release::mbid::ReleaseMBID;
use super::release_group::mbid::ReleaseGroupMBID;
use super::work::mbid::WorkMBID;

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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, IsVariant, Unwrap)]
pub enum RelationTarget {
    Artist(ArtistMBID),
    Area(),  //TODO
    Event(), //TODO
    Label(), //TODO
    Place(), //TODO
    Recording(RecordingMBID),
    Release(ReleaseMBID),
    ReleaseGroup(ReleaseGroupMBID),
    Series(), //TODO
    Url(),    //TODO
    Work(WorkMBID),
}
