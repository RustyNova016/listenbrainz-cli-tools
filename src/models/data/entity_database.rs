use std::sync::Arc;

use derive_getters::Getters;
use once_cell::sync::Lazy;

use crate::core::caching::entity_cache::EntityCache;

use super::listenbrainz::user_listens::UserListens;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug, Getters)]
pub struct EntityDatabase {
    user_listens: Arc<EntityCache<UserListens>>,
}

impl EntityDatabase {
    pub async fn remove(&self, id: &str) -> color_eyre::Result<()> {
        self.user_listens.remove(id).await?;

        Ok(())
    }
}

impl Default for EntityDatabase {
    fn default() -> Self {
        Self {
            user_listens: Arc::new(EntityCache::new("user_listens")),
        }
    }
}
