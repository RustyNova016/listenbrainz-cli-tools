use crate::core::entity_traits::{
    insertable::{Insertable, IsAutoInsertable},
    insertable_children::InsertableWithChildren,
};
use musicbrainz_rs::entity::artist::Artist;

impl InsertableWithChildren for Artist {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(recordings) = self.recordings.clone() {
            for recording in recordings {
                recording.insert_into_cache().await?;
            }
        }

        if let Some(releases) = self.releases.clone() {
            for release in releases {
                release.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}
