use chrono::{DateTime, Utc};
use listenbrainz::raw::Client;
use musicbrainz_db_lite::{
    connections::sqlite::SqliteClient, models::listenbrainz::listen::Listen,
};

use crate::utils::println_lis;

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
        println_lis(format!(
            "Getting listens from before: {} ({})",
            DateTime::from_timestamp(pull_ts.unwrap(), 0).unwrap(),
            pull_ts.unwrap(),
        ));
        pull_ts = Listen::execute_listen_fetch(client, &lb_client, user, pull_ts.unwrap()).await?;
    }

    Ok(())
}
