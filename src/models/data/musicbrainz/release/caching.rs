use super::media::Media;
use super::Release;
use crate::core::caching::disk_cache::DiskCacheWrapper;
use crate::core::caching::global_cache::GlobalCache;
use crate::core::entity_traits::cached_trait::CacheFromMusicbrainz;
use crate::core::entity_traits::has_cache::HasCache;
use crate::core::entity_traits::insert_external_entity_into_cache::InsertExternalEntityIntoCache;
use crate::core::entity_traits::merge::UpdateCachedEntity;
use crate::models::data::musicbrainz::HasMbid;
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use std::sync::Arc;

impl CacheFromMusicbrainz<ReleaseMS> for Release {
    fn insert_ms_with_id_into_cache(mbid: String, value: ReleaseMS) -> color_eyre::Result<()> {
        Self::set_or_update(mbid, value.clone().into())?;

        Media::insert_opt_ext_iter_into_cache(value.media.clone())?;

        Ok(())
    }
}

impl HasMbid for ReleaseMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

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

impl HasCache<String, Release> for Release {
    fn get_cache() -> Arc<DiskCacheWrapper<String, Release>> {
        GlobalCache::new().get_release_cache()
    }
}
