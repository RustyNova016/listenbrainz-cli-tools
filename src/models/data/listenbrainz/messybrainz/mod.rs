use listenbrainz::raw::response::UserListensListen;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessyBrainzData {
    pub msid: String,
    pub track_name: String,
    pub artist_name: String,
}

impl From<UserListensListen> for MessyBrainzData {
    fn from(value: UserListensListen) -> Self {
        Self {
            msid: value.recording_msid,
            track_name: value.track_metadata.track_name,
            artist_name: value.track_metadata.artist_name,
        }
    }
}
