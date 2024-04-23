use musicbrainz_rs::entity::artist_credit::ArtistCredit as ArtistCreditMS;

use super::ArtistCredit;

impl From<ArtistCreditMS> for ArtistCredit {
    fn from(value: ArtistCreditMS) -> Self {
        Self {
            artist: value.artist.id,
            joinphrase: value.joinphrase,
            name: value.name,
        }
    }
}
