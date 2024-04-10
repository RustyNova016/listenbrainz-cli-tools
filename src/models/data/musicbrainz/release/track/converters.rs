use musicbrainz_rs::entity::release::Track as TrackMS;

use super::Track;

impl From<TrackMS> for Track {
    fn from(value: TrackMS) -> Self {
        Self {
            id: value.id,
            length: value.length,
            number: value.number,
            position: value.position,
            recording: value.recording.id,
            title: value.title,
        }
    }
}
