use crate::core::entity_traits::insertable::InsertableAs;
use musicbrainz_rs::entity::{recording::Recording, release::Media as MediaMS};

impl InsertableAs<String, Recording> for MediaMS {
    async fn insert_into_cache_as(&self, _key: String) -> color_eyre::Result<()> {
        if let Some(tracks) = &self.tracks {
            for track in tracks {
                track.insert_into_cache_as("".to_string()).await?;
            }
        }

        Ok(())
    }
}
