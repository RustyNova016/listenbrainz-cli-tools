use super::Track;
use crate::core::entity_traits::cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId};
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::HasMbid;
use musicbrainz_rs::entity::release::Track as TrackMS;

impl HasMbid for TrackMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

impl CacheFromMusicbrainz<TrackMS> for Track {
    fn insert_ms_with_id_into_cache(_mbid: String, value: TrackMS) -> color_eyre::eyre::Result<()> {
        Recording::insert_ms_into_cache(value.recording)
    }
}
