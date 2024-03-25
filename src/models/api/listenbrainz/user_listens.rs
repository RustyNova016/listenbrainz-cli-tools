use chrono::{DateTime, Utc};
use derive_builder::Builder;
use listenbrainz::raw::response::UserListensResponse;

use crate::{
    models::{cache::listen_cache::ListenCache, data::listens::collection::UserListenCollection},
    utils::{extensions::UserListensPayloadExt, println_lis},
};

use super::ListenBrainzAPI;

impl ListenBrainzAPI {
    // Fetch the most listens it can, that have been listened before the provided date. Additionally save them to the cache
    fn fetch_before(
        &mut self,
        user: &str,
        before_date: DateTime<Utc>,
    ) -> color_eyre::Result<UserListensResponse> {
        println_lis(&format!(
            "Getting Listens from: {} ({})",
            before_date,
            before_date.timestamp()
        ));

        let result =
            self.api_client
                .user_listens(user, None, Some(before_date.timestamp()), Some(999))?;

        self.listen_cache
            .get_or_new_mut(user)
            .insert_api_return(result.payload.clone());

        Ok(result)
    }

    pub fn fetch_lastest_listens(&mut self, user: &str) -> color_eyre::Result<()> {
        let cache_of_user = self.listen_cache.get_or_new_mut(user);
        let operation_start = Utc::now();

        // We get the date of the latest listen
        let latest_cached_listen_date = cache_of_user
            .get_latest_cached_listen()
            .map(|cached_listen| cached_listen.listen_data.listened_at);

        // Prepare the loop variables
        let mut last_count = 1;
        let mut before_date = operation_start;

        // While we have still items, and that we aren't already reached the cached listens
        while last_count != 0
            && !latest_cached_listen_date.is_some_and(|cache_date| cache_date > before_date)
        {
            // We fetch a page of listens
            let result = self.fetch_before(user, before_date)?;

            // We put the new before date as the last listen's
            before_date = result
                .payload
                .get_date_of_oldest_listen_of_payload()
                .unwrap_or(operation_start);
            last_count = result.payload.listens.len();
        }

        Ok(())
    }

    pub fn get_cached_listens_of_user(&mut self, username: &str) -> UserListenCollection {
        self.listen_cache.get_or_new_mut(username).get_listens()
    }
}

#[derive(Debug, Builder)]
pub struct ListenFetchRequest {
    #[builder(setter(into))]
    client: ListenCache,

    #[builder(setter(into))]
    users: Vec<String>,

    #[builder(setter(into, strip_option), default = "true")]
    fetch_new: bool,

    #[builder(setter(into, strip_option), default = "false")]
    refresh_unlinked: bool,

    #[builder(setter(into, strip_option), default = "false")]
    refresh_all: bool,
}

impl ListenFetchRequest {}
