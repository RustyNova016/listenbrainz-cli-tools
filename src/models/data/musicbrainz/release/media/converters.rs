use musicbrainz_rs::entity::release::Media as MediaMS;

use crate::models::data::musicbrainz::release::track::Track;

use super::Media;

impl From<MediaMS> for Media {
    fn from(value: MediaMS) -> Self {
        Self {
            disc_count: value.disc_count,
            format: value.format,
            format_id: value.format_id,
            position: value.position,
            title: value.title,
            track_count: value.track_count,
            tracks: value
                .tracks
                .map(|tracks| tracks.into_iter().map(Track::from).collect()),
        }
    }
}
