use listenbrainz::raw::response::UserListensMBIDMapping;
use serde::{Deserialize, Serialize};

use crate::utils::extensions::UserListensMBIDMappingExt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingData {
    /// The MBID of the recordings
    pub recording_mbid: String,

    /// Name of the recording
    pub recording_name: String,

    /// Artists MBID
    pub artist_mbid: Vec<String>,

    /// Artist credits:
    pub artist_credit: Option<String>,
}

impl MappingData {
    pub fn get_recording_id(&self) -> &String {
        &self.recording_mbid
    }

    pub fn get_recording_name(&self) -> &String {
        &self.recording_name
    }

    pub fn get_artists_mbids(&self) -> &Vec<String> {
        &self.artist_mbid
    }
}

impl From<UserListensMBIDMapping> for MappingData {
    fn from(value: UserListensMBIDMapping) -> Self {
        Self {
            recording_mbid: value.recording_mbid.clone(),
            recording_name: value
                .recording_name
                .clone()
                .unwrap_or(format!("Unknown Track ({})", value.recording_mbid)),
            artist_mbid: value.artist_mbids.clone().unwrap_or_default(),
            artist_credit: value.get_artist_credit_as_string(),
        }
    }
}
