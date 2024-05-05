use std::sync::Arc;

use crate::core::caching::entity_cache::EntityCache;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::artist::Artist;

impl HasID for Artist {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Cached for Artist {
    fn get_cache() -> Arc<EntityCache<Self>>
    where
        Self: Sized,
    {
        ENTITY_DATABASE.artists()
    }
}

impl Updatable for Artist {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            name: newer.name,
        }
    }
}
