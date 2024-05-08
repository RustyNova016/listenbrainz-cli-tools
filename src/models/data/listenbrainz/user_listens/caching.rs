use std::sync::Arc;

use color_eyre::eyre::Ok;

use crate::core::caching::entity_cache::EntityCache;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::models::data::entity_database::ENTITY_DATABASE;

use super::UserListens;

impl HasID for UserListens {
    fn get_id(&self) -> String {
        self.username.to_string()
    }
}

impl Cached for UserListens {
    fn get_cache() -> Arc<EntityCache<Self>> {
        ENTITY_DATABASE.user_listens()
    }
}

impl UserListens {
    pub async fn get_from_cache(username: &str) -> color_eyre::Result<Option<Self>> {
        Self::get_cache().get(&username.to_lowercase()).await
    }

    pub async fn get_from_cache_or_new(username: &str) -> color_eyre::Result<Self> {
        Ok(Self::get_from_cache(username)
            .await?
            .unwrap_or_else(|| Self::new(username)))
    }
}
