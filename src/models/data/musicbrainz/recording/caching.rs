use std::sync::Arc;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::recording::Recording;

impl HasID for Recording {
    fn get_id(&self) -> String {
        self.id.to_string()
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

impl Updatable for Recording {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            title: newer.title,
            artist_credit: newer.artist_credit.or(self.artist_credit),
        }
    }
}
