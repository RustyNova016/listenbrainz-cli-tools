use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::insertable::InsertableAs;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;

impl Fetchable<String> for Artist {
    fn fetch(
        key: &String,
    ) -> impl std::future::Future<Output = color_eyre::Result<impl InsertableAs<String, Self>>> + Send
    where
        Self: Sized,
    {
        let key = key.clone();
        async move {
            println_mus(format!("Getting data for artist MBID: {}", &key));

            ArtistMS::fetch()
                .id(&key)
                .with_recordings()
                .execute()
                .await
                .context("Failed to fetch artist from MusicBrainz")
        }
    }
}
