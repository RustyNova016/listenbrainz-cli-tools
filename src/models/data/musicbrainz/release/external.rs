use musicbrainz_rs::entity::release::Release;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;

impl HasID for Release {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Release {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::release::Release::get_cache()
            .update(&key, self.clone().into())
            .await?;

        Ok(())
    }
}

impl InsertableWithChildren for Release {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(data) = self.artist_credit.clone() {
            for item in &data {
                item.insert_into_cache().await?;
            }
        }

        if let Some(data) = self.media.clone() {
            for item in &data {
                item.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}
