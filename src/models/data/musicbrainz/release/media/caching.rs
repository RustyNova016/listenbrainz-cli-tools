use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use musicbrainz_rs::entity::release::Media as MediaMS;

impl Insertable for MediaMS {
    async fn insert_into_cache_as(&self, _key: String) -> color_eyre::Result<()> {
        if let Some(tracks) = &self.tracks {
            for track in tracks {
                track.insert_into_cache_as(String::new()).await?;
            }
        }

        Ok(())
    }
}

impl HasID for MediaMS {
    fn get_id(&self) -> String {
        String::new()
    }
}
