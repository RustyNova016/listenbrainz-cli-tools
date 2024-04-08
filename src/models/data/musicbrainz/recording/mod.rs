
use crate::models::data::recording::Recording;
use color_eyre::{
    eyre::{eyre, Context, OptionExt},
    Result,
};

use super::{artist_credit::collection::ArtistCredits, HasMbid};

pub mod caching;
pub mod fetching;

impl HasMbid for Recording {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

impl Recording {
    pub fn get_artist_credits(&self) -> Option<ArtistCredits> {
        self.artist_credit.clone()
    }

    pub fn get_or_fetch_artist_credits(&self) -> Result<ArtistCredits> {
        Ok(match &self.get_artist_credits() {
            Some(credits) => credits.clone(),
            None => Self::get_or_fetch(self.get_mbid())
                .context("Couldn't fetch data from the API")?
                .get_artist_credits()
                .ok_or_eyre(eyre!("Artist credit is null after fetching from the API. Something wrong happened, as it should return a empty vec. Is there an include missing somewhere in the API call?"))?
        })
    }
}
