use std::sync::Arc;

use crate::models::{
    cache::{
        cached_trait::CacheFromMusicbrainz,
        disk_cache::DiskCacheWrapper,
        global_cache::GlobalCache,
        traits::{has_cache::HasCache, merge::UpdateCachedEntity, InsertExternalEntityIntoCache},
    },
    data::musicbrainz::HasMbid,
};

use super::{media::Media, Release};
use musicbrainz_rs::entity::release::Release as ReleaseMS;

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
