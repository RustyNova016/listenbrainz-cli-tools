use crate::core::entity_traits::insertable::{InsertableAs, IsAutoInsertableAs};
use crate::models::data::musicbrainz::recording::Recording;
use musicbrainz_rs::entity::release::Track as TrackMS;

impl InsertableAs<String, Recording> for TrackMS {
    fn insert_into_cache_as(
        &self,
        _key: String,
    ) -> impl std::future::Future<Output = color_eyre::Result<()>> + Send {
        self.recording.insert_into_cache()
    }
}
