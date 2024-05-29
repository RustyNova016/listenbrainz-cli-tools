pub mod collection;
pub mod converters;
pub mod external;
pub mod has_relationships;
pub mod inspections;
pub mod type_ids;

use super::artist::mbid::ArtistMBID;
use super::recording::mbid::RecordingMBID;
use super::release::mbid::ReleaseMBID;
use super::release_group::mbid::ReleaseGroupMBID;
use super::work::mbid::WorkMBID;
use crate::models::data::musicbrainz::url::mbid::URLMBID;
use chrono::NaiveDate;
use derive_getters::Getters;
use derive_more::IsVariant;
use derive_more::Unwrap;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

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

impl Relation {
    pub fn is_target_parent(&self) -> bool {
        let forward_is_child = match self.type_id.as_str() {
            //TODO: Find all of them
            // Lyrical quotation
            "c8283596-6f1f-42db-be9c-def66d387e78"
            // Musical quotaion
            | "c5decae0-535c-4730-aa5f-ab78eadd98ba"=> false,
            _ => true,
        };

        if &self.direction == "forward" {
            return !forward_is_child;
        }

        forward_is_child
    }
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
    Url(URLMBID),
    Work(WorkMBID),
}
