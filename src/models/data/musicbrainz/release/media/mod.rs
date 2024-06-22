use derive_getters::Getters;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::track::Track;

pub mod converters;
pub mod getters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Media {
    title: Option<String>,
    position: Option<u32>,
    track_count: u32,
    disc_count: Option<u32>,
    format_id: Option<String>,
    format: Option<String>,
    tracks: Option<Vec<Track>>,
}

impl Media {
    pub fn get_recording_mbids(&self) -> Option<Vec<RecordingMBID>> {
        self.tracks.as_ref().map(|tracks| {
            tracks
                .iter()
                .map(|track| track.get_recording_mbid())
                .collect_vec()
        })
    }
}
