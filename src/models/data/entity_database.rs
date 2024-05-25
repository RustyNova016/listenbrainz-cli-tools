use std::sync::Arc;

use derive_getters::Getters;
use once_cell::sync::Lazy;

use crate::core::caching::entity_cache::EntityCache;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;

use super::listenbrainz::user_listens::UserListens;
use super::musicbrainz::artist::Artist;
use super::musicbrainz::recording::Recording;
use super::musicbrainz::release::Release;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug, Getters)]
pub struct EntityDatabase {
    artists: Arc<EntityCache<Artist>>,
    releases: Arc<EntityCache<Release>>,
    recordings: Arc<EntityCache<Recording>>,
    release_groups: Arc<EntityCache<ReleaseGroup>>,

    user_listens: Arc<EntityCache<UserListens>>,
}

impl EntityDatabase {
    pub async fn remove(&self, id: &str) -> color_eyre::Result<()> {
        self.artists.remove(id).await?;
        self.releases.remove(id).await?;
        self.recordings.remove(id).await?;
        self.release_groups.remove(id).await?;
        self.user_listens.remove(id).await?;

        Ok(())
    }

    pub async fn invalidate_last_entries(
        &self,
        k: usize,
        keep_min: usize,
    ) -> color_eyre::Result<()> {
        self.artists.invalidate_last_entries(k, keep_min).await?;
        self.releases.invalidate_last_entries(k, keep_min).await?;
        self.recordings.invalidate_last_entries(k, keep_min).await?;
        self.release_groups
            .invalidate_last_entries(k, keep_min)
            .await?;

        Ok(())
    }
}

impl Default for EntityDatabase {
    fn default() -> Self {
        Self {
            artists: Arc::new(EntityCache::new("artists")),
            releases: Arc::new(EntityCache::new("releases")),
            recordings: Arc::new(EntityCache::new("recordings")),
            release_groups: Arc::new(EntityCache::new("release_groups")),

            user_listens: Arc::new(EntityCache::new("user_listens")),
        }
    }
}
