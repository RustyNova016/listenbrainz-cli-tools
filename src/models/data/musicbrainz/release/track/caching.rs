use musicbrainz_rs::entity::release::Track as TrackMS;

use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;

impl Insertable for TrackMS {
    fn insert_into_cache_as(
        &self,
        _key: String,
    ) -> impl std::future::Future<Output = color_eyre::Result<()>> + Send {
        self.recording.insert_into_cache()
    }
}

impl InsertableWithChildren for TrackMS {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        self.recording.insert_into_cache().await?;
        Ok(())
    }
}
