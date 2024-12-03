use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::database::get_db_client;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::println_cli;

#[cfg(not(test))]
use crate::utils::cli::await_next;

pub async fn lookup_recording(username: &str, id: RecordingMBID) -> color_eyre::Result<()> {
    let db = get_db_client().await;
    let conn = &mut *db.connection.acquire().await?;

    // Prefetch the listens. TODO: Merge specific rrecording fetching into query
    ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::None)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await?;

    let Some(recording) = Recording::fetch_and_save(conn, &id.to_string()).await? else {
        println_cli(format!("Couldn't find the recording with id: {id}"));
        return Ok(());
    };

    let listens = Listen::get_listens_of_recording_by_user(conn, username, recording.id).await?;

    let coupled = RecordingWithListens::from_listencollection(conn, listens.into())
        .await
        .expect("Couldn't load recording")
        .into_values()
        .collect_vec()
        .pop()
        .unwrap();

    println!(
        "{}",
        coupled
            .get_lookup_report_async(conn)
            .await
            .expect("Couldn't generate lookup report")
    );

    #[cfg(not(test))]
    await_next();

    Ok(())
}
