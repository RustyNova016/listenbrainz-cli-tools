use musicbrainz_rs::entity::artist_credit::ArtistCredit as ArtistCreditMS;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;

impl Insertable for ArtistCreditMS {
    fn insert_into_cache_as(
        &self,
        key: String,
    ) -> impl std::future::Future<Output = color_eyre::Result<()>> + Send {
        self.artist.insert_into_cache_as(key)
    }
}

impl HasID for ArtistCreditMS {
    fn get_id(&self) -> String {
        self.artist.id.to_string()
    }
}
