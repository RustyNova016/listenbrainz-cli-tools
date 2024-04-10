use std::future::Future;

use crate::models::cache::cached_trait::CacheFromMusicbrainz;
use crate::models::cache::global_cache::GlobalCache;
use crate::models::cache::traits::merge::UpdateCachedEntity;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::HasMbid;
use cached::DiskCacheError;
use itertools::Itertools;
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use tokio::task::JoinHandle;

impl Recording {
    pub fn get_from_cache(mbid: &str) -> Result<Option<Recording>, cached::DiskCacheError> {
        GlobalCache::new().get_recording(mbid)
    }

    pub fn insert_into_cache(
        mbid: String,
        value: Recording,
    ) -> Result<Option<Recording>, cached::DiskCacheError> {
        GlobalCache::new().insert_recording(mbid, value)
    }
}

impl CacheFromMusicbrainz<RecordingMS> for Recording {
    fn insert_ms_with_id_into_cache(mbid: String, value: RecordingMS) -> color_eyre::Result<()> {
        Self::insert_into_cache(mbid, value.clone().into())?;

        Ok(())
    }
}

impl HasMbid for RecordingMS {
    fn get_mbid(&self) -> &str {
        &self.id
    }
}

pub trait HasRecordingID {
    fn get_recording_mbid(&self) -> &str;

    fn get_recording(&self) -> Result<Option<Recording>, DiskCacheError> {
        GlobalCache::new().get_recording(&self.get_recording_mbid())
    }

    fn get_recording_or_fetch(
        &self,
    ) -> impl Future<Output = Result<Recording, color_eyre::eyre::Error>> {
        Recording::get_or_fetch(self.get_recording_mbid())
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
                    let recording = Recording::get_or_fetch(&id).await;
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
