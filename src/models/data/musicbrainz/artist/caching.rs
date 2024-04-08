use musicbrainz_rs::entity::artist::Artist as ArtistMS;

use crate::models::data::musicbrainz::HasMbid;
use crate::models::{
    cache::{
        cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId},
        global_cache::GlobalCache,
    },
    data::recording::{Artist, Recording},
};

impl Artist {
    pub fn insert_into_cache(
        mbid: String,
        value: Artist,
    ) -> Result<Option<Artist>, cached::DiskCacheError> {
        GlobalCache::new().insert_artist(mbid, value)
    }
}

impl CacheFromMusicbrainz<ArtistMS> for Artist {
    fn insert_ms_with_id_into_cache(mbid: String, value: ArtistMS) -> color_eyre::Result<()> {
        Self::insert_into_cache(mbid, value.clone().into())?;

        if let Some(recordings) = value.recordings {
            Recording::insert_ms_iter_into_cache(recordings)?;
        }

        Ok(())
    }
}

impl HasMbid for ArtistMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}
