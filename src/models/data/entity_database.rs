use derive_getters::Getters;
use std::sync::Arc;

use once_cell::sync::Lazy;

use super::listenbrainz::user_listens::UserListens;
use super::musicbrainz::artist::Artist;
use super::musicbrainz::recording::Recording;
use super::musicbrainz::release::Release;
use super::musicbrainz::work::Work;
use crate::core::caching::entity_cache::EntityCache;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug, Getters)]
pub struct EntityDatabase {
    artists: Arc<EntityCache<Artist>>,
    releases: Arc<EntityCache<Release>>,
    recordings: Arc<EntityCache<Recording>>,
    release_groups: Arc<EntityCache<ReleaseGroup>>,
    works: Arc<EntityCache<Work>>,

    user_listens: Arc<EntityCache<UserListens>>,
}

impl Default for EntityDatabase {
    fn default() -> Self {
        Self {
            artists: Arc::new(EntityCache::new("artists")),
            releases: Arc::new(EntityCache::new("releases")),
            recordings: Arc::new(EntityCache::new("recordings")),
            release_groups: Arc::new(EntityCache::new("release_groups")),
            works: Arc::new(EntityCache::new("works")),

            user_listens: Arc::new(EntityCache::new("user_listens")),
        }
    }
}
