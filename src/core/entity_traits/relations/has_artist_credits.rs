use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use color_eyre::eyre::{eyre, Context, OptionExt};

pub trait HasArtistCredits: HasID + Cached + Fetchable {
    fn get_artist_credits(&self) -> &Option<ArtistCredits>;

    async fn get_or_fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        Ok(match &self.get_artist_credits() {
            Some(credits) => credits.clone(),
            None => {
                Self::get_cache().fetch_and_save(self.get_id().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any release with the MBID"))?
                    .get_artist_credits()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credit is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}
