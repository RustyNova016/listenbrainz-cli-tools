use crate::core::caching::entity_cache::EntityCache;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use std::sync::Arc;

impl HasID for ReleaseGroup {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Cached for ReleaseGroup {
    fn get_cache() -> Arc<EntityCache<Self>>
    where
        Self: Sized,
    {
        ENTITY_DATABASE.release_groups().clone()
    }
}

impl Updatable for ReleaseGroup {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            secondary_types: newer.secondary_types,
            secondary_type_ids: newer.secondary_type_ids,
            disambiguation: newer.disambiguation,
            title: newer.title,
            primary_type_id: newer.primary_type_id.or(self.primary_type_id),
            first_release_date: newer.first_release_date.or(self.first_release_date),
            primary_type: newer.primary_type.or(self.primary_type),
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            genres: newer.genres.or(self.genres),
            releases: newer.releases.or(self.releases),
            annotation: newer.annotation.or(self.annotation),
            artist_credit: newer.artist_credit.or(self.artist_credit),
        }
    }
}
