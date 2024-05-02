use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable_children::InsertChildren;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;

impl Fetchable for Artist {
    #[allow(refining_impl_trait)]
    async fn fetch(key: &str) -> color_eyre::Result<InsertChildren<ArtistMS>> {
        println_mus(format!("Getting data for artist MBID: {}", &key));

        Ok(ArtistMS::fetch()
            .id(key)
            .with_recordings()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?
            .into())
    }
}
