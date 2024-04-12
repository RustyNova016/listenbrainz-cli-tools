use crate::models::{
    cache::{
        cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId},
        global_cache::GlobalCache,
        traits::{merge::UpdateCachedEntity, InsertExternalEntityIntoCache},
    },
    data::musicbrainz::recording::Recording,
};

use super::{media::Media, track, Release};
use musicbrainz_rs::entity::release::Release as ReleaseMS;

impl Release {
    pub fn insert_into_cache(
        mbid: &str,
        value: Release,
    ) -> Result<Option<Release>, cached::DiskCacheError> {
        GlobalCache::new().insert_release(&mbid, value)
    }
}

impl CacheFromMusicbrainz<ReleaseMS> for Release {
    fn insert_ms_with_id_into_cache(mbid: String, value: ReleaseMS) -> color_eyre::Result<()> {
        Self::insert_into_cache(&mbid, value.clone().into())?;

        Media::insert_opt_ext_iter_into_cache(value.media.clone())?;

        Ok(())
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
