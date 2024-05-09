use super::Release;
use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable_children::InsertChildren;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use musicbrainz_rs::Fetch;

impl Fetchable for Release {
    #[allow(refining_impl_trait)]
    async fn fetch(key: &str) -> color_eyre::Result<InsertChildren<ReleaseMS>> {
        println_mus(format!("Getting data for release MBID: {}", &key));

        Ok(ReleaseMS::fetch()
            .id(key)
            .with_recordings()
            .with_artists()
            .with_artist_credits()
            .with_release_groups()
            .with_artist_credits()
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .execute()
            .await
            .context("Failed to fetch release from MusicBrainz")?
            .into())
    }
}
