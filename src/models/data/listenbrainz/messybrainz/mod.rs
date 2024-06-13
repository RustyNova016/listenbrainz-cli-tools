use listenbrainz::raw::response::UserListensListen;
use serde::{Deserialize, Serialize};

use crate::utils::extensions::listenbrainz_ext::UserListensTrackMetadataExt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessyBrainzData {
    pub msid: String,
    pub track_name: String,
    pub artist_name: String,
    pub origin_url: Option<String>,
}

impl From<UserListensListen> for MessyBrainzData {
    fn from(value: UserListensListen) -> Self {
        Self {
            msid: value.recording_msid,
            track_name: value.track_metadata.track_name.clone(),
            artist_name: value.track_metadata.artist_name.clone(),
            origin_url: value
                .track_metadata
                .get_additional_string_metadata("origin_url")
                .cloned(),
        }
    }
}
