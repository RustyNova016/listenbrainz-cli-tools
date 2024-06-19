use std::ops::Deref;
use std::sync::Arc;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::musicbrainz::recording::Recording;

use chrono::DateTime;
use chrono::Utc;
use derive_getters::Getters;
use derive_new::new;

use super::recording_id::RecordingIDWithListens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingWithListens {
    recording: Arc<Recording>,
    recording_with_id: RecordingIDWithListens,
}

impl RecordingWithListens {
    pub fn new(recording: Arc<Recording>, listens: ListenCollection) -> Self {
        let recording_with_id = RecordingIDWithListens::new(recording.id().clone(), listens);

        Self {
            recording,
            recording_with_id,
        }
    }
}

impl Deref for RecordingWithListens {
    type Target = RecordingIDWithListens;

    fn deref(&self) -> &RecordingIDWithListens {
        &self.recording_with_id
    }
}
