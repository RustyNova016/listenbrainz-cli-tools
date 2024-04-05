use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::{entity::artist::Artist as ArtistMS, Fetch};
use std::sync::Arc;

use crate::{
    models::{
        cache::{cached_trait::CacheFromMusicbrainzAutoId, global_cache::GlobalCache},
        data::recording::Artist,
    },
    utils::println_mus,
};

impl Artist {
    pub fn get_or_fetch(mbid: &str) -> color_eyre::Result<Arc<Self>> {
        match GlobalCache::new().get_artist(mbid) {
            Some(val) => Ok(val),
            None => Self::fetch(mbid),
        }
    }

    fn fetch(mbid: &str) -> color_eyre::Result<Arc<Self>> {
        println_mus(format!("Getting data for artist MBID: {}", &mbid));
        let msreturn = ArtistMS::fetch()
            .id(&mbid)
            .with_recordings()
            .execute()
            .context("Failed to fetch artist from MusicBrainz")?;
        Self::insert_ms_into_cache(msreturn);

        // The element have been inserted above, so it should be safe to unwrap
        Ok(GlobalCache::new().get_artist(&mbid).unwrap())
    }
}
