use crate::models::data::musicbrainz::recording::caching::HasRecordingID;

use super::Track;

impl HasRecordingID for Track {
    fn get_recording_mbid(&self) -> &str {
        &self.recording
    }
}
