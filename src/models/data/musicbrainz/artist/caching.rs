use std::sync::Arc;

use musicbrainz_rs::entity::artist::Artist as ArtistMS;

use crate::models::{
    cache::{
        cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId},
        global_cache::GlobalCache,
    },
    data::recording::{Artist, Recording},
};
use crate::models::data::musicbrainz::HasMbid;

impl Artist {
    fn get_cached(key: &String) -> Option<Arc<Artist>> {
        GlobalCache::new().get_artist(&key)
    }

    pub fn insert_into_cache(mbid: String, value: Artist) -> Option<Arc<Artist>> {
        GlobalCache::new().insert_artist(mbid.into(), value.into())
    }
}

impl CacheFromMusicbrainz<ArtistMS> for Artist {
    fn insert_ms_with_id_into_cache(mbid: String, value: ArtistMS) {
        Self::insert_into_cache(mbid, value.clone().into());
        Self::insert_into_cache(value.get_mbid().to_string(), value.clone().into());

        if let Some(recordings) = value.recordings {
            Recording::insert_ms_iter_into_cache(recordings)
        }
    }
}

impl HasMbid for ArtistMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}
