use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

pub mod converters;
pub mod getters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Track {
    pub recording: String,
    pub title: String,
    pub number: String,
    pub length: Option<u32>,
    pub position: u32,
    pub id: String,
}

impl Track {
    pub fn get_recording_mbid(&self) -> RecordingMBID {
        RecordingMBID::from(self.recording.clone())
    }
}
