use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::{entity::recording::Recording as RecordingMS, Fetch};


use crate::{
    models::{
        cache::{cached_trait::CacheFromMusicbrainzAutoId, global_cache::GlobalCache},
        data::recording::Recording,
    },
    utils::println_mus,
};

impl Recording {
    pub fn get_or_fetch(mbid: &str) -> color_eyre::Result<Self> {
        //println!("Recording from cache: {:?}", GlobalCache::new().get_recording(mbid));
        match GlobalCache::new().get_recording(mbid)? {
            Some(val) => Ok(val),
            None => Self::fetch(mbid),
        }
    }

    pub(super) fn fetch(mbid: &str) -> color_eyre::Result<Self> {
        println_mus(format!("Getting data for recording MBID: {}", &mbid));

        let msreturn = RecordingMS::fetch()
            .id(mbid)
            .with_artists()
            .execute()
            .context("Failed to fetch recording from MusicBrainz")?;

        Self::insert_ms_with_alias_into_cache(mbid.to_string(), msreturn)?;

        // The element have been inserted above, so it should be safe to unwrap the option
        Ok(GlobalCache::new().get_recording(mbid)?.unwrap())
    }
}
