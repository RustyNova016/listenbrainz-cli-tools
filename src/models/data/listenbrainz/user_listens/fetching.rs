use chrono::{DateTime, TimeDelta, Utc};
use color_eyre::eyre::eyre;
use color_eyre::Report;
use listenbrainz::raw::response::UserListensResponse;
use listenbrainz::raw::Client;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::core::display::progress_bar::ProgressBarCli;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::insertable::Insertable;
use crate::models::config::global_config::CONFIG;
use crate::utils::extensions::UserListensPayloadExt;
use crate::utils::{println_cli, println_lis};

use super::UserListens;

static FETCH_COUNT: Lazy<RwLock<u64>> = Lazy::new(|| RwLock::new(999));

impl UserListens {

}

impl Insertable for UserListensResponse {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        let mut user_listens = UserListens::get_cache()
            .get(&key)
            .await?
            .unwrap_or_else(|| UserListens::new(&key));

        user_listens.refresh_timerange(self.payload.clone());

        UserListens::get_cache()
            .set(&key.to_lowercase(), user_listens)
            .await?;
        Ok(())
    }
}
