use crate::models::{
    cache::cached_trait::CacheFromMusicbrainzAutoId,
    data::recording::{Artist},
};
use musicbrainz_rs::entity::artist_credit::ArtistCredit as ArtistCreditMS;

use super::ArtistCredit;

impl ArtistCredit {
    pub fn insert_ms_artist_into_cache(value: ArtistCreditMS) {
        Artist::insert_ms_into_cache(value.artist)
    }

    pub fn insert_ms_artist_iter_into_cache<I: IntoIterator<Item = ArtistCreditMS>>(values: I) {
        values
            .into_iter()
            .for_each(|value| Self::insert_ms_artist_into_cache(value))
    }
}
