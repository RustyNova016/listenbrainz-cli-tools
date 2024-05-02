use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable::InsertableAs;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::println_mus;
use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;

impl Fetchable for Recording {
    async fn fetch(key: &String) -> color_eyre::Result<impl InsertableAs<Self>>
    where
        Self: Sized,
    {
        println_mus(format!("Getting data for recording MBID: {}", &key));

        Ok(RecordingMS::fetch()
            .id(key)
            .with_artists()
            .with_releases()
            .execute()
            .await
            .context("Failed to fetch recording from MusicBrainz")?)
    }
}
