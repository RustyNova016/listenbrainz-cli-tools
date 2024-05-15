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
        ENTITY_DATABASE.recordings().clone()
    }
}

impl Updatable for Recording {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            title: newer.title,
            artist_credit: newer.artist_credit.or(self.artist_credit),
            releases: newer.releases.or(self.releases),
            isrcs: newer.isrcs.or(self.isrcs),
            disambiguation: newer.disambiguation.or(self.disambiguation),
            tags: newer.tags.or(self.tags),
            video: newer.video.or(self.video),
            length: newer.length.or(self.length),
            annotation: newer.annotation.or(self.annotation),
            genres: newer.genres.or(self.genres),
            aliases: self.aliases,
            relations: newer.relations.or(self.relations),
        }
    }
}
