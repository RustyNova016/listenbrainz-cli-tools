use crate::models::{
    cache::cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId},
    data::musicbrainz::{recording::Recording, HasMbid},
};
use musicbrainz_rs::entity::release::Track as TrackMS;

use super::Track;

impl HasMbid for TrackMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

impl CacheFromMusicbrainz<TrackMS> for Track {
    fn insert_ms_with_id_into_cache(mbid: String, value: TrackMS) -> color_eyre::eyre::Result<()> {
        Recording::insert_ms_into_cache(value.recording)
    }
}
