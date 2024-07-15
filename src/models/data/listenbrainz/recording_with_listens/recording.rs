use std::ops::Deref;
use std::sync::Arc;

use crate::models::data::listenbrainz::listen::collection::mapped_primary_collection::PrimaryListenCollection;
use crate::models::data::musicbrainz::recording::Recording;

use derive_getters::Getters;

use super::recording_id::RecordingIDWithListens;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingWithListens {
    recording: Arc<Recording>,
    recording_with_id: RecordingIDWithListens,
}

impl RecordingWithListens {
    pub fn new_from_unfiltered(
        recording: Arc<Recording>,
        listens: &PrimaryListenCollection,
    ) -> Self {
        let recording_with_id =
            RecordingIDWithListens::new_from_unfiltered(recording.get_mbid().clone(), listens);

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
