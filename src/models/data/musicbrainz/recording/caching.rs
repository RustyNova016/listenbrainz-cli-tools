use std::sync::Arc;

use crate::models::api::GetFromCacheOrFetch;
use crate::models::cache::cached_trait::CacheFromMusicbrainz;
use crate::models::cache::disk_cache::DiskCacheWrapper;
use crate::models::cache::global_cache::GlobalCache;
use crate::models::cache::traits::has_cache::HasCache;
use crate::models::cache::traits::merge::UpdateCachedEntity;

use crate::models::data::musicbrainz::artist_credit::ArtistCredit;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::HasMbid;
use cached::DiskCacheError;
use itertools::Itertools;
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use tokio::task::JoinHandle;

impl CacheFromMusicbrainz<RecordingMS> for Recording {
    fn insert_ms_with_id_into_cache(mbid: String, value: RecordingMS) -> color_eyre::Result<()> {
        Self::set_or_update(mbid, value.clone().into())?;

        if let Some(data) = value.artist_credit.clone() {
            ArtistCredit::insert_ms_artist_iter_into_cache(data)?;
        }

        Ok(())
    }
}

impl HasMbid for RecordingMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

pub trait HasRecordingIDs {
    fn get_recording_mbids(&self) -> Vec<String>;

    fn get_recordings(&self) -> Result<Vec<(String, Option<Recording>)>, DiskCacheError> {
        let cache = GlobalCache::new();
        self.get_recording_mbids()
            .into_iter()
            .map(|id| {
                let recording = cache.get_recording(&id);
                recording.map(|record| (id, record))
            })
            .collect()
    }

    fn get_recordings_or_fetch(&self) -> Vec<JoinHandle<color_eyre::Result<(String, Recording)>>> {
        self.get_recording_mbids()
            .into_iter()
            .map(|id| {
                let id = id.clone();
                tokio::spawn(async move {
                    let recording = Recording::get_cached_or_fetch(&id).await;
                    recording.map(|record| (id, record))
                })
            })
            .collect_vec()
    }
}

impl UpdateCachedEntity for Recording {
    fn update_entity(self, new: Self) -> Self {
        Self {
            artist_credit: new.artist_credit.or(self.artist_credit),
            id: new.id,
            title: new.title,
        }
    }
}

impl HasCache<String, Recording> for Recording {
    fn get_cache() -> Arc<DiskCacheWrapper<String, Recording>> {
        GlobalCache::new().get_recording_cache()
    }
}
