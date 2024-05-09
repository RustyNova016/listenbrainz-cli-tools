use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use musicbrainz_rs::entity::release::Media as MediaMS;

impl HasID for MediaMS {
    fn get_id(&self) -> String {
        String::new()
    }
}

impl Insertable for MediaMS {
    async fn insert_into_cache_as(&self, _key: String) -> color_eyre::Result<()> {
        Ok(())
    }
}

impl InsertableWithChildren for MediaMS {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(tracks) = &self.tracks {
            for track in tracks {
                track.insert_with_children(String::new()).await?;
            }
        }

        Ok(())
    }
}
