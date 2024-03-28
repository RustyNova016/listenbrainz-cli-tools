use crate::models::data::recording::ArtistCredit;

use super::MusicBrainzAPI;

impl MusicBrainzAPI {
    pub(super) fn insert_artist_credit(&self, value: ArtistCredit) {
        self.insert_artist(value.artist.id.clone(), value.artist)
    }

    pub(super) fn insert_artist_credits(&self, value: Vec<ArtistCredit>) {
        for artist_credit_element in value {
            self.insert_artist_credit(artist_credit_element)
        }
    }
}
