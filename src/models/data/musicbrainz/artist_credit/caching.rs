use crate::models::{cache::cached_trait::CacheFromMusicbrainzAutoId, data::recording::Artist};
use musicbrainz_rs::entity::artist_credit::ArtistCredit as ArtistCreditMS;

use super::ArtistCredit;

impl ArtistCredit {
    pub fn insert_ms_artist_into_cache(value: ArtistCreditMS) -> color_eyre::Result<()> {
        Artist::insert_ms_into_cache(value.artist)?;
        Ok(())
    }

    pub fn insert_ms_artist_iter_into_cache<I: IntoIterator<Item = ArtistCreditMS>>(
        values: I,
    ) -> color_eyre::Result<()> {
        values
            .into_iter().try_for_each(Self::insert_ms_artist_into_cache)
    }
}
