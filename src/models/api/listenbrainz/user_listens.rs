use chrono::{DateTime, TimeDelta, Utc};
use color_eyre::eyre::Ok;

use listenbrainz::raw::response::UserListensResponse;

use crate::{
    models::data::listens::collection::UserListenCollection,
    utils::{extensions::UserListensPayloadExt, println_cli, println_lis, traits::VecWrapper},
};

use super::ListenBrainzAPI;

impl ListenBrainzAPI {
    /// Fetch the most listens it can, that have been listened before the provided date. Additionally save them to the cache
    pub fn fetch_before(
        &mut self,
        user: &str,
        before_date: DateTime<Utc>,
    ) -> color_eyre::Result<UserListensResponse> {
        println_lis(&format!(
            "Getting listens from before: {} ({})",
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

    /// Fetch the latest listens that aren't yet in the cache. If it fetched more than needed, entries will get updated
    ///
    /// If the cache is empty, then it will fill it completly
    pub fn update_lastest_listens(&mut self, username: &str) -> color_eyre::Result<()> {
        let cache_of_user = self.listen_cache.get_or_new_mut(username);
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
            let result = self.fetch_before(username, before_date)?;

            // We put the new before date as the last listen's
            before_date = result
                .payload
                .get_date_of_oldest_listen_of_payload()
                .unwrap_or(operation_start);
            last_count = result.payload.listens.len();
        }

        Ok(())
    }

    /// Fetch the listens of an user using a combination of api calls and cache
    pub fn fetch_listens_of_user_cached(
        &mut self,
        username: &str,
    ) -> color_eyre::Result<UserListenCollection> {
        self.update_lastest_listens(username)?;
        self.save_cache()?;
        Ok(self.get_cached_listens_of_user(username))
    }

    /// Refetch all the unlinked listens of an user and recache them
    pub fn update_unlinked_of_user(&mut self, username: &str) -> color_eyre::Result<()> {
        // We first get all the unlinked cached
        let mut unlinkeds = self
            .get_cached_listens_of_user(username)
            .get_unmapped_listens();

        while unlinkeds.len() > 0 {
            let refresh_target = unlinkeds
                .get_latest_listen()
                .expect("Couldn't fetch listen");

            let result = self
                .fetch_before(
                    username,
                    refresh_target.listened_at + TimeDelta::new(1, 0).unwrap(),
                )?
                .payload;

            unlinkeds.remove_period(
                result
                    .get_date_of_oldest_listen_of_payload()
                    .unwrap_or(Utc::now()),
                result
                    .get_date_of_latest_listen_of_payload()
                    .unwrap_or(Utc::now()),
                true,
            )
        }

        Ok(())
    }

    /// Get all the unlinked listens of an user
    pub fn fetch_unlinked_of_user(
        &mut self,
        username: &str,
    ) -> color_eyre::Result<UserListenCollection> {
        // Get new listens
        println_cli("Getting new listens...");
        self.update_lastest_listens(username)?;
        self.save_cache()?;

        // Refresh the unlinkeds
        println_cli("Refreshing unlinked listens...");
        self.update_unlinked_of_user(username)?;
        self.save_cache()?;

        Ok(self
            .get_cached_listens_of_user(username)
            .get_unmapped_listens())
    }

    pub fn get_cached_listens_of_user(&mut self, username: &str) -> UserListenCollection {
        self.listen_cache.get_or_new_mut(username).get_listens()
    }
}

//#[derive(Debug, Builder)]
//pub struct ListenFetchRequest {
//    #[builder(setter(into))]
//    client: ListenCache,
//
//    #[builder(setter(into))]
//    users: Vec<String>,
//
//    #[builder(setter(into, strip_option), default = "true")]
//    fetch_new: bool,
//
//    #[builder(setter(into, strip_option), default = "false")]
//    refresh_unlinked: bool,
//
//    #[builder(setter(into, strip_option), default = "false")]
//    refresh_all: bool,
//}
//
//impl ListenFetchRequest {}
//
