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
use crate::utils::extensions::UserListensPayloadExt;
use crate::utils::{println_cli, println_lis};

use super::UserListens;

static FETCH_COUNT: Lazy<RwLock<u64>> = Lazy::new(|| RwLock::new(999));

impl UserListens {
    pub async fn get_user_with_refresh(username: &str) -> color_eyre::Result<Self> {
        println_cli("Getting new user listens...");
        Self::fetch_latest(username).await?;

        println_cli("Updating unmapped listens...");
        //Self::update_unlinked_of_user(username).await?; //TODO: Put back on

        Ok(Self::get_from_cache(username)
            .await
            .expect("Couldn't get listen that were inserted")
            .expect("Couldn't get listen that were inserted"))
    }

    /// Fetch the most listens it can, that have been listened before the provided date. Additionally, save them to the cache
    pub async fn fetch_before(
        user: &str,
        before_date: DateTime<Utc>,
    ) -> color_eyre::Result<UserListensResponse> {
        println_lis(format!(
            "Getting listens from before: {} ({})",
            before_date,
            before_date.timestamp()
        ));

        // We give it 20 tries
        for _i in 0..20 {
            let result = Client::new().user_listens(
                user,
                None,
                Some(before_date.timestamp()),
                Some(*FETCH_COUNT.read().await),
            );

            match result {
                Ok(data) => {
                    data.insert_into_cache_as(user.to_lowercase()).await?;
                    return Ok(data);
                }
                Err(listenbrainz::Error::Http(_val)) => {
                    println_lis("Io error, retrying");
                    let count = *FETCH_COUNT.read().await;

                    // Retry with shorter count
                    *FETCH_COUNT.write().await = count.div_ceil(2);
                }
                Err(val) => return Err(Report::from(val)),
            }
        }

        Err(eyre!("Maximum tries exceded"))
    }

    /// Fetch the latest listens that aren't yet in the cache. If it fetched more than needed, entries will get updated
    ///
    /// If the cache is empty, then it will fill it completely
    pub async fn fetch_latest(username: &str) -> color_eyre::Result<()> {
        let operation_start = Utc::now();

        let latest_cached_listen_date = Self::get_from_cache(username)
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
        let mut unlinkeds = Self::get_from_cache_or_new(username)
            .await?
            .get_unmapped_listens();
        let start_count = unlinkeds.len();

        let progress_bar =
            ProgressBarCli::new(unlinkeds.len() as u64, Some("Updating unmapped listens"));

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
                    .unwrap_or_else(Utc::now),
                &result
                    .get_date_of_latest_listen_of_payload()
                    .unwrap_or_else(Utc::now),
                true,
            );

            progress_bar.set_position((start_count - unlinkeds.len()).try_into().unwrap());
        }

        Ok(())
    }
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
