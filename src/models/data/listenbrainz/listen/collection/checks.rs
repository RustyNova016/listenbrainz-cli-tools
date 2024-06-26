use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::ListenCollection;

impl ListenCollection {
    /// Checks if the listencollection only has listens of this recording
    pub fn has_only_recording(&self, recording_id: &RecordingMBID) -> bool {
        self.iter().all(|listen| listen.is_mapped_to(recording_id))
    }
}
