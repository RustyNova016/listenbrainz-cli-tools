use color_eyre::eyre::{eyre, Context, OptionExt};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::error::Error;

pub trait HasArtistCredits<K: IsMbid<Self> + Serialize + DeserializeOwned>:
    HasID + MBCached<K>
{
    fn get_artist_credits(&self) -> &Option<ArtistCredits>;

    async fn get_or_fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        if let Some(data) = self.get_artist_credits() {
            return Ok(data.clone());
        }

        Ok(match &self.get_artist_credits() {
            Some(credits) => credits.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .get_artist_credits()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credits field is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }

    /// Force fetch the related entity
    async fn fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        let refreshed = self
            .refresh()
            .await
            .context("Couldn't fetch data from the API")?;

        Ok(refreshed
            .get_artist_credits()
            .as_ref()
            .ok_or(Error::NoneAfterFetch(
                "artist_credits".to_string(),
                self.get_id(),
            ))
            .cloned()?)
    }
}
