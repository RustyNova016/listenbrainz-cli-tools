use color_eyre::eyre::{Context, eyre, OptionExt};
use crate::core::entity_traits::has_id::HasID;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::release::Release;

impl Release {
    pub async fn get_or_fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        Ok(match &self.artist_credit {
            Some(credits) => credits.clone(),
            None => {
                ENTITY_DATABASE.releases().fetch_and_save(self.get_id().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any release with the MBID"))?
                    .artist_credit
                    .ok_or_eyre(eyre!(format!("Artist credit is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested release ID is: {}", &self.id)))?
            }
        })
    }
}