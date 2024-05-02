use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use crate::core::entity_traits::merge::UpdateCachedEntity;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::recording::Recording;
use chrono::format::Item;
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use std::sync::Arc;

impl UpdateCachedEntity for Recording {
    fn update_entity(self, new: Self) -> Self {
        Self {
            artist_credit: new.artist_credit.or(self.artist_credit),
            id: new.id,
            title: new.title,
            releases: new.releases.or(self.releases),
        }
    }
}

impl Cached for Recording {
    fn get_cache() -> Arc<crate::core::caching::entity_cache::EntityCache<Self>>
    where
        Self: Sized,
    {
        ENTITY_DATABASE.recordings()
    }
}

impl Insertable for RecordingMS {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        Recording::get_cache()
            .set(&key, self.clone().into())
            .await?;

        Ok(())
    }
}

impl HasID for Recording {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}
