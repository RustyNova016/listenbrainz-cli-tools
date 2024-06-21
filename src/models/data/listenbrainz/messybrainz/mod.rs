pub mod msid;
use std::collections::HashMap;

use derive_getters::Getters;
use listenbrainz::raw::response::UserListensListen;
use serde::{Deserialize, Serialize};

use crate::utils::extensions::listenbrainz_ext::UserListensTrackMetadataExt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct MessyBrainzData {
    pub msid: String,
    pub track_name: String,
    pub artist_name: String,
    release_name: Option<String>,
    pub origin_url: Option<String>,
    additional_info: HashMap<String, serde_json::Value>
}

impl MessyBrainzData {
    pub fn get_field(&self, name: &str) -> Option<String> {
        if name == "msid" {return Some(self.msid.clone());}
        if name == "track_name" {return Some(self.track_name.clone());}
        if name == "artist_name" {return Some(self.artist_name.clone());}
        if name == "release_name" {return self.release_name.as_ref().map(|val| val.to_string());}
        
        self.additional_info.get(name).map(|val| val.to_string())
    } 
}

impl From<UserListensListen> for MessyBrainzData {
    fn from(value: UserListensListen) -> Self {
        Self {
            msid: value.recording_msid,
            track_name: value.track_metadata.track_name.clone(),
            artist_name: value.track_metadata.artist_name.clone(),
            release_name: value.track_metadata.release_name.clone(),
            origin_url: value
                .track_metadata
                .get_additional_string_metadata("origin_url")
                .cloned(),
                additional_info: value.track_metadata.additional_info,

        }
    }
}
