pub mod converters;
use std::collections::HashMap;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

pub mod msid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct MessyBrainzData {
    pub msid: String,
    pub track_name: String,
    pub artist_name: String,
    pub(super) release_name: Option<String>,
    pub origin_url: Option<String>,
    pub(super) additional_info: HashMap<String, serde_json::Value>,
}

impl MessyBrainzData {
    pub fn get_field(&self, name: &str) -> Option<String> {
        if name == "msid" {
            return Some(self.msid.clone());
        }
        if name == "track_name" {
            return Some(self.track_name.clone());
        }
        if name == "artist_name" {
            return Some(self.artist_name.clone());
        }
        if name == "release_name" {
            return self.release_name.as_ref().map(|val| val.to_string());
        }

        self.additional_info.get(name).map(|val| val.to_string())
    }
}
