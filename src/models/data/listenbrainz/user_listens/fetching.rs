use chrono::{DateTime, TimeDelta, Utc};

use indicatif::ProgressBar;
use listenbrainz::raw::response::UserListensResponse;
use listenbrainz::raw::Client;

use crate::models::api::FetchAPI;
use crate::models::cache::global_cache::GlobalCache;
use crate::utils::extensions::UserListensPayloadExt;
use crate::utils::{println_lis, Logger};

use super::UserListens;

impl UserListens {
    /// Fetch the most listens it can, that have been listened before the provided date. Additionally save them to the cache
    pub fn fetch_before(
        user: &str,
        before_date: DateTime<Utc>,
    ) -> color_eyre::Result<UserListensResponse> {
        println_lis(format!(
            "Getting listens from before: {} ({})",
            before_date,
            before_date.timestamp()
        ));

        let result =
            Client::new().user_listens(user, None, Some(before_date.timestamp()), Some(999))?;

        GlobalCache::new().insert_lb_listen_payload(result.payload.clone())?;

        Ok(result)
    }

    /// Fetch the latest listens that aren't yet in the cache. If it fetched more than needed, entries will get updated
    ///
    /// If the cache is empty, then it will fill it completly
    pub fn fetch_latest(username: &str) -> color_eyre::Result<()> {
        let operation_start = Utc::now();

        let latest_cached_listen_date = GlobalCache::new()
            .get_user_listens(&username.to_lowercase())?
            .and_then(|user_listens| user_listens.listens.get_latest_listen())
            .map(|listen| *listen.get_listened_at());

        // Prepare the loop variables
        let mut last_count = 1;
        let mut before_date = operation_start;

        // While we have still items, and that we aren't already reached the cached listens
        while last_count != 0
            && !latest_cached_listen_date.is_some_and(|cache_date| cache_date > before_date)
        {
            // We fetch a page of listens
            let result = Self::fetch_before(username, before_date)?;

            // We put the new before date as the last listen's
            before_date = result
                .payload
                .get_date_of_oldest_listen_of_payload()
                .unwrap_or(operation_start);
            last_count = result.payload.listens.len();
        }

        Ok(())
    }

    /// Refetch all the unlinked listens of a user and recache them
    pub fn update_unlinked_of_user(username: &str) -> color_eyre::Result<()> {
        // We first get all the unlinked cached
        let mut unlinkeds = GlobalCache::new()
            .get_user_listens_or_empty(username)?
            .get_unmapped_listens();
        let start_count = unlinkeds.len();

        let progress_bar = ProgressBar::new(unlinkeds.len().try_into().unwrap());
        Logger::set_global_overide(progress_bar.clone());

        while unlinkeds.len() > 0 {
            let refresh_target = unlinkeds
                .get_latest_listen()
                .expect("Couldn't fetch listen");

            let result = Self::fetch_before(
                username,
                refresh_target.listened_at + TimeDelta::new(1, 0).unwrap(),
            )?
            .payload;

            unlinkeds.remove_timerange(
                &result
                    .get_date_of_oldest_listen_of_payload()
                    .unwrap_or(Utc::now()),
                &result
                    .get_date_of_latest_listen_of_payload()
                    .unwrap_or(Utc::now()),
                true,
            );

            progress_bar.set_position((start_count - unlinkeds.len()).try_into().unwrap());
        }

        Logger::clear_global_overide();

        Ok(())
    }
}

impl FetchAPI<String, UserListens> for UserListens {
    async fn fetch_and_insert(key: &String) -> color_eyre::Result<UserListens> {
        GlobalCache::new().get_user_listens_with_refresh(key)
    }
}
