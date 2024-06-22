use std::collections::HashMap;

use chrono::NaiveDate;
use derive_getters::Getters;
use derive_more::IsVariant;
use derive_more::Unwrap;
use serde::Deserialize;
use serde::Serialize;

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::mbid::MBID;

use super::artist::mbid::ArtistMBID;
use super::recording::mbid::RecordingMBID;
use super::release::mbid::ReleaseMBID;
use super::release_group::mbid::ReleaseGroupMBID;
use super::work::mbid::WorkMBID;

pub mod converters;
pub mod external;

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
    Url(),    //TODO
    Work(WorkMBID),
}

impl RelationTarget {
    pub fn into_mbid_safe(self) -> Option<MBID> {
        match self {
            RelationTarget::Artist(val) => Some(val.into_mbid()),
            RelationTarget::Area() => None,
            RelationTarget::Event() => None,
            RelationTarget::Label() => None,
            RelationTarget::Place() => None,
            RelationTarget::Recording(val) => Some(val.into_mbid()),
            RelationTarget::Release(val) => Some(val.into_mbid()),
            RelationTarget::ReleaseGroup(val) => Some(val.into_mbid()),
            RelationTarget::Series() => None,
            RelationTarget::Url() => None,
            RelationTarget::Work(val) => Some(val.into_mbid()),
        }
    }
}
