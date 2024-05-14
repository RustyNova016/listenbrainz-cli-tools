use musicbrainz_rs::entity::work::Work;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use crate::core::entity_traits::insertable_children::InsertableWithChildren;

impl HasID for Work {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Work {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::work::Work::get_cache()
            .update(&key, self.clone().into())
            .await
    }
}

impl InsertableWithChildren for Work {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        Ok(())
    }
}
