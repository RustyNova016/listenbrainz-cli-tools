use color_eyre::eyre::Context;
use musicbrainz_rs::Fetch;

use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable_children::InsertChildren;
use crate::utils::println_mus;
use musicbrainz_rs::entity::work::Work as WorkMS;

use super::Work;

impl Fetchable for Work {
    #[allow(refining_impl_trait)]
    async fn fetch(key: &str) -> color_eyre::Result<InsertChildren<WorkMS>> {
        println_mus(format!("Getting data for work MBID: {}", &key));

        Ok(WorkMS::fetch()
            .id(key)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_label_relations()
            .with_ratings()
            .with_recording_relations()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .execute()
            .await
            .context("Failed to fetch work from MusicBrainz")?
            .into())
    }
}
