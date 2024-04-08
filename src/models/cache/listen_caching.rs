use cached::DiskCacheError;
use cached::IOCached;
use listenbrainz::raw::response::UserListensPayload;

use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::utils::println_cli;

use super::static_cache::StaticCache;

impl StaticCache {
    /// Retrieve the user listens of a specific user
    pub fn get_user_listens(&self, key: &str) -> Result<Option<UserListens>, DiskCacheError> {
        self.listens.cache_get(&key.to_lowercase())
    }

    pub fn get_user_listens_or_empty(&self, key: &str) -> Result<UserListens, DiskCacheError> {
        Ok(self
            .listens
            .cache_get(&key.to_lowercase())?
            .unwrap_or(UserListens::new(key)))
    }

    fn insert_user_listens(
        &self,
        user_listens: UserListens,
    ) -> Result<Option<UserListens>, DiskCacheError> {
        self.listens
            .cache_set(user_listens.get_user().to_string(), user_listens)
    }

    pub fn get_user_listens_with_refresh(
        &self,
        key: &str,
    ) -> color_eyre::Result<Option<UserListens>> {
        println_cli("Getting new user listens...");
        UserListens::fetch_latest(key)?;

        println_cli("Updating unmapped listens...");
        UserListens::update_unlinked_of_user(key)?;
        Ok(self.get_user_listens(key)?)
    }

    pub fn insert_lb_listen_payload(
        &self,
        payload: UserListensPayload,
    ) -> Result<Option<UserListens>, DiskCacheError> {
        let mut user_listens = self.get_user_listens_or_empty(&payload.user_id)?;

        user_listens.refresh_timerange(payload);

        self.insert_user_listens(user_listens)
    }
}
