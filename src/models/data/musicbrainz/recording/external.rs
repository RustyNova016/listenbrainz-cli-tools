use musicbrainz_rs::entity::recording::Recording;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::{
    has_id::HasID,
    insertable::{Insertable, IsAutoInsertable},
    insertable_children::InsertableWithChildren,
};

impl HasID for Recording {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Recording {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::recording::Recording::get_cache()
            .update(&key, self.clone().into())
            .await?;

        Ok(())
    }
}

impl InsertableWithChildren for Recording {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(data) = self.artist_credit.clone() {
            for item in data.iter() {
                item.insert_into_cache().await?;
            }
        }

        if let Some(data) = self.releases.clone() {
            for item in data.iter() {
                item.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}
