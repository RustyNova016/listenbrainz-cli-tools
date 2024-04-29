use color_eyre::eyre::Ok;

use crate::core::caching::entity_cache::EntityCache;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::InsertableAs;
use crate::core::entity_traits::merge::UpdateCachedEntity;
use crate::models::data::entity_database::ENTITY_DATABASE;
use std::sync::Arc;

use super::UserListens;

impl UpdateCachedEntity for UserListens {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}

impl Cached<String> for UserListens {
    fn get_cache() -> Arc<EntityCache<String, UserListens>> {
        ENTITY_DATABASE.user_listens()
    }
}

impl InsertableAs<String, UserListens> for UserListens {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        UserListens::get_cache().set(&key, self.clone()).await?;
        Ok(())
    }
}

impl HasID<String> for UserListens {
    fn get_id(&self) -> String {
        self.username.to_string()
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
