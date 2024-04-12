use super::Release;
use crate::models::cache::cached_trait::CacheFromMusicbrainzAutoId;

use crate::models::cache::traits::has_cache::HasCache;
use crate::{models::api::FetchAPI, utils::println_mus};
use color_eyre::eyre::Context;
use musicbrainz_rs::{entity::release::Release as ReleaseMS, Fetch};

impl FetchAPI<String, Release> for Release {
    fn fetch_and_insert(
        key: &String,
    ) -> impl std::future::Future<Output = color_eyre::Result<Release>> {
        let key = key.clone();
        async move {
            println_mus(format!("Getting data for artist MBID: {}", &key));
            let msreturn = ReleaseMS::fetch()
                .id(&key)
                .with_recordings()
                .execute()
                .await
                .context("Failed to fetch artist from MusicBrainz")?;

            Self::insert_ms_into_cache(msreturn)?;

            // The element have been inserted above, so it should be safe to unwrap the option
            Ok(Self::get_from_cache(&key)?.unwrap())
        }
    }
}
