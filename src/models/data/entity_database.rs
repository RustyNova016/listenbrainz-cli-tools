use std::sync::Arc;

use once_cell::sync::Lazy;

use super::musicbrainz::{artist::Artist, recording::Recording, release::Release};
use crate::core::caching::entity_cache::EntityCache;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug)]
pub struct EntityDatabase {
    artists: Arc<EntityCache<String, Artist>>,
    releases: Arc<EntityCache<String, Release>>,
    recordings: Arc<EntityCache<String, Recording>>,
}

impl Default for EntityDatabase {
    fn default() -> Self {
        Self {
            artists: Arc::new(EntityCache::new("artists")),
            releases: Arc::new(EntityCache::new("releases")),
            recordings: Arc::new(EntityCache::new("recordings")),
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
}
