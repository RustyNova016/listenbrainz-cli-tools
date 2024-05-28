use super::Recording;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::HasMbid;
use color_eyre::eyre::{eyre, Context, OptionExt};

impl Recording {
    pub async fn get_or_fetch_releases_ids(&self) -> color_eyre::Result<Vec<ReleaseMBID>> {
        Ok(match &self.releases {
            Some(releases) => releases.clone(),
            None => {
                ENTITY_DATABASE.recordings().fetch_and_save(self.get_mbid().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any recording with the MBID"))?
                    .releases
                    .ok_or_eyre(eyre!(format!("Releases is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        })
    }
}

impl HasArtistCredits for Recording {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}
