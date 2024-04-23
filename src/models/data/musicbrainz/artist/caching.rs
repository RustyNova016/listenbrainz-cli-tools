use std::sync::Arc;

use musicbrainz_rs::entity::artist::Artist as ArtistMS;

use crate::models::cache::cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId};
use crate::models::cache::disk_cache::DiskCacheWrapper;
use crate::models::cache::global_cache::GlobalCache;
use crate::models::cache::traits::has_cache::HasCache;
use crate::models::cache::traits::merge::UpdateCachedEntity;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::HasMbid;

impl CacheFromMusicbrainz<ArtistMS> for Artist {
    fn insert_ms_with_id_into_cache(mbid: String, value: ArtistMS) -> color_eyre::Result<()> {
        Self::set_or_update(mbid, value.clone().into())?;

        if let Some(recordings) = value.recordings {
            Recording::insert_ms_iter_into_cache(recordings)?;
        }

        Ok(())
    }
}

impl HasMbid for ArtistMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

impl UpdateCachedEntity for Artist {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}

impl HasCache<String, Artist> for Artist {
    fn get_cache() -> Arc<DiskCacheWrapper<String, Artist>> {
        GlobalCache::new().get_artist_cache()
    }
}
