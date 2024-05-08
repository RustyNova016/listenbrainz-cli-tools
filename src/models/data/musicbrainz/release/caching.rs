use std::sync::Arc;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::entity_database::ENTITY_DATABASE;

use super::Release;

impl HasID for Release {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Cached for Release {
    fn get_cache() -> Arc<crate::core::caching::entity_cache::EntityCache<Self>>
    where
        Self: Sized,
    {
        ENTITY_DATABASE.releases().clone()
    }
}

impl Updatable for Release {
    fn update(self, newer: Self) -> Self {
        Self {
            annotation: newer.annotation.or(self.annotation),
            barcode: newer.barcode.or(self.barcode),
            country: newer.country.or(self.country),
            disambiguation: newer.disambiguation.or(self.disambiguation),
            media: newer.media.or(self.media),
            packaging_id: newer.packaging_id.or(self.packaging_id),
            status_id: newer.status_id.or(self.status_id),
            title: newer.title,
            id: newer.id,
            artist_credit: newer.artist_credit.or(self.artist_credit),
        }
    }
}
