use std::sync::Arc;

use derive_getters::Getters;
use once_cell::sync::Lazy;
use tokio::try_join;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::cli::cache::ClearTarget;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;

use super::musicbrainz::artist::Artist;
use super::musicbrainz::mbid::any_mbid::AnyMBIDType;
use super::musicbrainz::mbid::generic_mbid::NaiveID;
use super::musicbrainz::recording::Recording;
use super::musicbrainz::release::Release;
use super::musicbrainz::work::Work;

pub(crate) static MUSICBRAINZ_DATABASE: Lazy<Arc<MusicBrainzDatabase>> =
    Lazy::new(|| Arc::new(MusicBrainzDatabase::default()));

#[derive(Debug, Getters)]
pub struct MusicBrainzDatabase {
    artists: Arc<MusicbrainzCache<Artist>>,
    releases: Arc<MusicbrainzCache<Release>>,
    recordings: Arc<MusicbrainzCache<Recording>>,
    release_groups: Arc<MusicbrainzCache<ReleaseGroup>>,
    works: Arc<MusicbrainzCache<Work>>,
}

impl MusicBrainzDatabase {
    pub async fn remove(&self, id: &AnyMBIDType<NaiveID>) -> color_eyre::Result<()> {
        match id {
            AnyMBIDType::Artist(id) => self.artists.remove(id).await?,
            AnyMBIDType::Release(id) => self.releases.remove(id).await?,
            AnyMBIDType::Recording(id) => self.recordings.remove(id).await?,
            AnyMBIDType::ReleaseGroup(id) => self.release_groups.remove(id).await?,
            AnyMBIDType::Work(id) => self.works.remove(id).await?,
        }

        Ok(())
    }

    pub async fn invalidate_last_entries(
        &self,
        k: usize,
        keep_min: usize,
    ) -> color_eyre::Result<()> {
        try_join!(
            self.artists.invalidate_last_entries(k, keep_min),
            self.releases.invalidate_last_entries(k, keep_min),
            self.recordings.invalidate_last_entries(k, keep_min),
            self.release_groups.invalidate_last_entries(k, keep_min),
            self.works.invalidate_last_entries(k, keep_min)
        )?;

        Ok(())
    }

    // pub async fn add_alias(&self, alias: &MBID, main: &MBID) -> color_eyre::Result<()> {
    //     // Check if both are the same variant
    //     if discriminant(alias) != discriminant(main) {
    //         return Err(Report::from(Error::MBIDAliasError(
    //             alias.clone(),
    //             main.clone(),
    //         )));
    //     }

    //     let main = main.clone();

    //     match alias {
    //         MBID::Artist(alias) => {
    //             self.artists
    //                 .insert_alias(alias, &main.unwrap_artist())
    //                 .await?;
    //         }
    //         MBID::Release(alias) => {
    //             self.releases
    //                 .insert_alias(alias, &main.unwrap_release())
    //                 .await?;
    //         }
    //         MBID::Work(alias) => self.works.insert_alias(alias, &main.unwrap_work()).await?,
    //         MBID::ReleaseGroup(alias) => {
    //             self.release_groups
    //                 .insert_alias(alias, &main.unwrap_release_group())
    //                 .await?;
    //         }
    //         MBID::Recording(alias) => {
    //             self.recordings
    //                 .insert_alias(alias, &main.unwrap_recording())
    //                 .await?;
    //         }
    //     }

    //     Ok(())
    // }

    pub async fn clear(&self, target: &ClearTarget) -> cacache::Result<()> {
        match target {
            ClearTarget::All => {
                let _ = try_join!(
                    self.artists.clear(),
                    self.releases.clear(),
                    self.recordings.clear(),
                    self.release_groups.clear(),
                    self.works.clear()
                )?;
            }
        }

        Ok(())
    }
}

impl Default for MusicBrainzDatabase {
    fn default() -> Self {
        Self {
            artists: Arc::new(MusicbrainzCache::new("artists")),
            releases: Arc::new(MusicbrainzCache::new("releases")),
            recordings: Arc::new(MusicbrainzCache::new("recordings")),
            release_groups: Arc::new(MusicbrainzCache::new("release_groups")),
            works: Arc::new(MusicbrainzCache::new("works")),
        }
    }
}
