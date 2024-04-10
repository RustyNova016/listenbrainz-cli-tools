use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;

use crate::models::cache::cached_trait::CacheFromMusicbrainzAutoId;
use crate::models::cache::global_cache::GlobalCache;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::println_mus;

impl Artist {
    pub async fn get_or_fetch(mbid: &str) -> color_eyre::Result<Self> {
        match GlobalCache::new().get_artist(mbid)? {
            Some(val) => Ok(val),
            None => Self::fetch(mbid).await,
        }
    }

    async fn fetch(mbid: &str) -> color_eyre::Result<Self> {
        println_mus(format!("Getting data for artist MBID: {}", &mbid));
        let msreturn = ArtistMS::fetch()
            .id(mbid)
            .with_recordings()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?;

        Self::insert_ms_into_cache(msreturn)?;

        // The element have been inserted above, so it should be safe to unwrap the option
        Ok(GlobalCache::new().get_artist(mbid)?.unwrap())
    }
}
