use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::insertable::Insertable;
use crate::utils::extensions::UserListensPayloadExt;
use crate::utils::{println_cli, println_lis, Logger};
use chrono::{DateTime, TimeDelta, Utc};
use indicatif::ProgressBar;
use listenbrainz::raw::response::UserListensResponse;
use listenbrainz::raw::Client;

use super::UserListens;

impl UserListens {
    pub async fn get_user_with_refresh(username: &str) -> color_eyre::Result<UserListens> {
        println_cli("Getting new user listens...");
        UserListens::fetch_latest(username).await?;

        println_cli("Updating unmapped listens...");
        UserListens::update_unlinked_of_user(username).await?;

        Ok(Self::get_from_cache(username)
            .await
            .expect("Couldn't get listen that were inserted")
            .expect("Couldn't get listen that were inserted"))
    }

    /// Fetch the most listens it can, that have been listened before the provided date. Additionally save them to the cache
    pub async fn fetch_before(
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

        result.insert_into_cache_as(user.to_lowercase()).await?;

        Ok(result)
    }

    /// Fetch the latest listens that aren't yet in the cache. If it fetched more than needed, entries will get updated
    ///
    /// If the cache is empty, then it will fill it completly
    pub async fn fetch_latest(username: &str) -> color_eyre::Result<()> {
        let operation_start = Utc::now();

        let latest_cached_listen_date = UserListens::get_from_cache(username)
            .await?
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
            let result = Self::fetch_before(username, before_date).await?;

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
    pub async fn update_unlinked_of_user(username: &str) -> color_eyre::Result<()> {
        // We first get all the unlinked cached
        let mut unlinkeds = UserListens::get_from_cache_or_new(username)
            .await?
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
            )
            .await?
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

impl Insertable for UserListensResponse {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        let mut user_listens = UserListens::get_cache()
            .get(&key)
            .await?
            .unwrap_or(UserListens::new(&key));

        user_listens.refresh_timerange(self.payload.clone());

        UserListens::get_cache()
            .set(&key.to_lowercase(), user_listens)
            .await?;
        Ok(())
    }
}
