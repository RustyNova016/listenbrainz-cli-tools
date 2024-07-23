use crate::utils::extensions::listenbrainz_ext::UserListensTrackMetadataExt;
use listenbrainz::raw::response::UserListensListen;

use super::MessyBrainzData;

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
