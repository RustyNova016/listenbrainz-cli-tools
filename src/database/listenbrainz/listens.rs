use chrono::{DateTime, Utc};
use listenbrainz::raw::Client;
use macon::Builder;
use musicbrainz_db_lite::connections::sqlite::SqliteClient;
use musicbrainz_db_lite::database::client::DBClient;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use sqlx::SqliteConnection;

use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli::global_progress_bar::PG_FETCHING;
use crate::utils::println_cli;

/// Fetch the latest listens for the provided user. If the user has no listens, it will do a full listen fetch.
pub async fn fetch_latest_listens_of_user(
    client: &SqliteClient,
    user: &str,
) -> Result<(), musicbrainz_db_lite::Error> {
    let latest_listen_ts =
        Listen::get_latest_listen_of_user(&mut *client.as_sqlx_pool().acquire().await?, user)
            .await?
            .map(|v| v.listened_at);
    let mut pull_ts = Some(Utc::now().timestamp());

    let lb_client = Client::new();

    // This loop has two possible states.
    // - Fresh dump:
    //     `latest_listen_ts` is none. We loop until `save_listen_payload_in_transaction` tell us it's over
    //
    // - Incremental dump:
    //     `latest_listen_ts` is set. We loop until pull_ts is before the latest listen
    while (latest_listen_ts.is_none() && pull_ts.is_some())
        || (latest_listen_ts.is_some_and(|a| pull_ts.is_some_and(|b| a <= b)))
    {
        println_cli(format!(
            "Getting listens from before: {} ({})",
            DateTime::from_timestamp(pull_ts.unwrap(), 0).unwrap(),
            pull_ts.unwrap(),
        ));
        pull_ts = Listen::execute_listen_fetch(client, &lb_client, user, pull_ts.unwrap()).await?;
    }

    Ok(())
}



#[derive(Builder)]
pub struct ListenFetchQuery {
    #[builder(Default=!)]
    user: String,

    fetch_recordings_redirects: bool,

    returns: ListenFetchQueryReturn,
}

impl ListenFetchQuery {
    pub async fn fetch(self, client: &DBClient) -> Result<ListenCollection, crate::Error> {
        fetch_latest_listens_of_user(client.as_welds_client(), &self.user).await?;
        let conn = &mut *client.as_sqlx_pool().acquire().await?;

        if self.fetch_recordings_redirects {
            Self::fetch_recordings_redirects(conn, &self.user).await?;
        }

        match self.returns {
            ListenFetchQueryReturn::Mapped => Ok(ListenCollection::new(
                Listen::get_mapped_listen_of_user(conn, &self.user).await?,
            )),
            ListenFetchQueryReturn::None => Ok(ListenCollection::default()),
        }
    }

    async fn fetch_recordings_redirects(
        conn: &mut SqliteConnection,
        user: &str,
    ) -> Result<(), crate::Error> {
        let unfetched = Listen::get_unfetched_recordings_of_user(conn, user).await?;
        let subm = PG_FETCHING.get_submitter(unfetched.len() as u64);
        
        for id in unfetched {
            Recording::get_or_fetch(conn, &id).await?;
            subm.inc(1);
        }

        Ok(())
    }
}

pub enum ListenFetchQueryReturn {
    Mapped,
    None,
}
