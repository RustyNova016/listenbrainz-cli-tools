use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::recording::Recording;

impl Recording {
    pub fn get_credited_artists_ids(&self) -> Option<Vec<ArtistMBID>> {
        self.artist_credit()
            .as_ref()
            .map(|credits| credits.get_artist_ids())
    }
}
