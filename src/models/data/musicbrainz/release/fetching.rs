use super::Release;
use crate::core::entity_traits::fetchable::Fetchable;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use musicbrainz_rs::Fetch;

impl Fetchable for Release {
    async fn fetch(
        key: &String,
    ) -> color_eyre::Result<impl crate::core::entity_traits::insertable::InsertableAs<Self>>
    where
        Self: Sized,
    {
        println_mus(format!("Getting data for artist MBID: {}", &key));

        ReleaseMS::fetch()
            .id(key)
            .with_recordings()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")
    }
}
