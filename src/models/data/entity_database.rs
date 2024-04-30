use std::sync::Arc;

use once_cell::sync::Lazy;

use super::{
    listenbrainz::user_listens::UserListens,
    musicbrainz::{artist::Artist, recording::Recording, release::Release},
};
use crate::core::caching::entity_cache::EntityCache;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug)]
pub struct EntityDatabase {
    artists: Arc<EntityCache<String, Artist>>,
    releases: Arc<EntityCache<String, Release>>,
    recordings: Arc<EntityCache<String, Recording>>,

    user_listens: Arc<EntityCache<String, UserListens>>,
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
    pub fn artists(&self) -> Arc<EntityCache<String, Artist>> {
        self.artists.clone()
    }

    pub fn releases(&self) -> Arc<EntityCache<String, Release>> {
        self.releases.clone()
    }

    pub fn recordings(&self) -> Arc<EntityCache<String, Recording>> {
        self.recordings.clone()
    }

    pub fn user_listens(&self) -> Arc<EntityCache<String, UserListens>> {
        self.user_listens.clone()
    }
}
