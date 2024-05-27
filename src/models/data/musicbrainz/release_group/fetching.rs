use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable_children::InsertChildren;
use crate::core::fetching::global_fetch_limiter::MB_FETCH_LIMITER;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;
use musicbrainz_rs::Fetch;

impl Fetchable for ReleaseGroup {
    #[allow(refining_impl_trait)]
    async fn fetch(key: &str) -> color_eyre::Result<InsertChildren<ReleaseGroupMS>> {
        let _permit = MB_FETCH_LIMITER.acquire().await?;

        println_mus(format!("Getting data for release group MBID: {}", &key));

        Ok(ReleaseGroupMS::fetch()
            .id(key)
            .with_releases()
            .execute()
            .await
            .context("Failed to fetch release group from MusicBrainz")?
            .into())
    }
}
