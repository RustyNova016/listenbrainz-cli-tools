use std::sync::Arc;

use once_cell::sync::Lazy;

use super::listenbrainz::user_listens::UserListens;
use super::musicbrainz::artist::Artist;
use super::musicbrainz::recording::Recording;
use super::musicbrainz::release::Release;
use crate::core::caching::entity_cache::EntityCache;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug)]
pub struct EntityDatabase {
    artists: Arc<EntityCache<Artist>>,
    releases: Arc<EntityCache<Release>>,
    recordings: Arc<EntityCache<Recording>>,

    user_listens: Arc<EntityCache<UserListens>>,
}

impl Default for EntityDatabase {
    fn default() -> Self {
        Self {
            artists: Arc::new(EntityCache::new("artists")),
            releases: Arc::new(EntityCache::new("releases")),
            recordings: Arc::new(EntityCache::new("recordings")),

            user_listens: Arc::new(EntityCache::new("user_listens")),
        }
    }
}

impl EntityDatabase {
    pub fn artists(&self) -> Arc<EntityCache<Artist>> {
        self.artists.clone()
    }

    pub fn releases(&self) -> Arc<EntityCache<Release>> {
        self.releases.clone()
    }

    pub fn recordings(&self) -> Arc<EntityCache<Recording>> {
        self.recordings.clone()
    }

    pub fn user_listens(&self) -> Arc<EntityCache<UserListens>> {
        self.user_listens.clone()
    }
}