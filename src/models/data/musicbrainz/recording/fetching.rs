use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable_children::InsertChildren;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::println_mus;
use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;

impl Fetchable for Recording {
    #[allow(refining_impl_trait)]
    async fn fetch(key: &str) -> color_eyre::Result<InsertChildren<RecordingMS>> {
        println_mus(format!("Getting data for recording MBID: {}", &key));

        Ok(RecordingMS::fetch()
            .id(key)
            .with_artists()
            .with_releases()
            .with_work_relations()
            .with_aliases()
            .with_work_level_relations()
            .execute()
            .await
            .context("Failed to fetch recording from MusicBrainz")?
            .into())
    }
}
