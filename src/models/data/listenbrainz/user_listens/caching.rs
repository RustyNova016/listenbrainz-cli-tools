use std::sync::Arc;

use color_eyre::eyre::Ok;

use crate::core::caching::entity_cache::EntityCache;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use crate::models::data::entity_database::ENTITY_DATABASE;

use super::UserListens;

impl HasID for UserListens {
    fn get_id(&self) -> String {
        self.username.to_string()
    }
}

impl Cached for UserListens {
    fn get_cache() -> Arc<EntityCache<UserListens>> {
        ENTITY_DATABASE.user_listens()
    }
}

impl Insertable for UserListens {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        UserListens::get_cache().set(&key, self.clone()).await?;
        Ok(())
    }
}

impl UserListens {
    pub async fn get_from_cache(username: &str) -> color_eyre::Result<Option<UserListens>> {
        Self::get_cache().get(&username.to_lowercase()).await
    }

    pub async fn get_from_cache_or_new(username: &str) -> color_eyre::Result<UserListens> {
        Ok(Self::get_from_cache(username)
            .await?
            .unwrap_or(UserListens::new(username)))
    }
}
