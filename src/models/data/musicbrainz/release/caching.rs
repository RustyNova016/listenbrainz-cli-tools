use super::Release;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use crate::core::entity_traits::merge::UpdateCachedEntity;
use crate::models::data::entity_database::ENTITY_DATABASE;
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use std::sync::Arc;

impl UpdateCachedEntity for Release {
    fn update_entity(self, new: Self) -> Self {
        Self {
            annotation: new.annotation.or(self.annotation),
            barcode: new.barcode.or(self.barcode),
            country: new.country.or(self.country),
            disambiguation: new.disambiguation.or(self.disambiguation),
            id: new.id,
            media: new.media.or(self.media),
            packaging_id: new.packaging_id.or(self.packaging_id),
            status_id: new.status_id.or(self.status_id),
            title: new.title,
        }
    }
}

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
        ENTITY_DATABASE.releases()
    }
}

impl Insertable for ReleaseMS {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        Release::get_cache().set(&key, self.clone().into()).await?;

        if let Some(tracks) = &self.media {
            for track in tracks {
                track.insert_into_cache_as("".to_string()).await?;
            }
        }

        Ok(())
    }
}
