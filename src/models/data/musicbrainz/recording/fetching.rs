use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;

use crate::models::api::FetchAPI;
use crate::models::cache::cached_trait::CacheFromMusicbrainzAutoId;
use crate::models::cache::global_cache::GlobalCache;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::println_mus;

impl FetchAPI<String, Recording> for Recording {
    fn fetch_and_insert(
        key: &String,
    ) -> impl std::future::Future<Output = color_eyre::Result<Recording>> {
        let key = key.clone();
        async move {
            println_mus(format!("Getting data for recording MBID: {}", &key));

            let msreturn = RecordingMS::fetch()
                .id(&key)
                .with_artists()
                .with_releases()
                .execute()
                .await
                .context("Failed to fetch recording from MusicBrainz")?;

            Self::insert_ms_with_alias_into_cache(key.to_string(), msreturn)?;

            // The element have been inserted above, so it should be safe to unwrap the option
            Ok(GlobalCache::new().get_recording(&key)?.unwrap())
        }
    }
}
