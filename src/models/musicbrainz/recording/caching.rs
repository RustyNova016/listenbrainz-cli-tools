use std::sync::Arc;

use crate::models::{
    cache::{cached_trait::CacheFromMusicbrainz, global_cache::GlobalCache},
    data::recording::Recording,
    musicbrainz::HasMbid,
};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;

impl Recording {
    fn get_cached(key: &String) -> Option<Arc<Recording>> {
        GlobalCache::new().get_recording(key)
    }

    pub fn insert_into_cache(mbid: String, value: Recording) -> Option<Arc<Recording>> {
        GlobalCache::new().insert_recording(mbid.into(), value.into())
    }
}

impl CacheFromMusicbrainz<RecordingMS> for Recording {
    fn insert_ms_with_id_into_cache(mbid: String, value: RecordingMS) {
        Self::insert_into_cache(mbid, value.clone().into());
        Self::insert_into_cache(value.get_mbid().to_string(), value.clone().into());
    }
}

impl HasMbid for RecordingMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}
