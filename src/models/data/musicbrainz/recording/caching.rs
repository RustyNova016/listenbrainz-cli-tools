

use crate::models::data::musicbrainz::HasMbid;
use crate::models::{
    cache::{cached_trait::CacheFromMusicbrainz, global_cache::GlobalCache},
    data::recording::Recording,
};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;

impl Recording {
    pub fn insert_into_cache(
        mbid: String,
        value: Recording,
    ) -> Result<Option<Recording>, cached::DiskCacheError> {
        GlobalCache::new().insert_recording(mbid, value)
    }
}

impl CacheFromMusicbrainz<RecordingMS> for Recording {
    fn insert_ms_with_id_into_cache(mbid: String, value: RecordingMS) -> color_eyre::Result<()> {
        Self::insert_into_cache(mbid, value.clone().into())?;
        Ok(())
    }
}

impl HasMbid for RecordingMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}
