use futures::stream;
use futures::StreamExt;
use itertools::Itertools;

use crate::api::listenbrainz::global_listen_counts::get_global_listen_counts;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::radio::collector::RadioCollector;
use crate::datastructures::radio::seeders::listens::ListenSeeder;
use crate::datastructures::radio::sorters::underrated::underrated_sorter;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::playlist::PlaylistStub;
use crate::utils::println_cli;

pub async fn underrated_mix(
    conn: &mut sqlx::SqliteConnection,
    seeder: ListenSeeder,
    collector: RadioCollector,
    token: &str,
) -> color_eyre::Result<()> {
    let username = seeder.username().clone();

    println_cli("[Seeding] Getting listens");

    // Get the seeder
    let recordings = seeder.seed(conn).await.expect("Couldn't find seed listens");

    // Get the all time listens
    let user_listens = ListenFetchQuery::builder()
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await
        .expect("Couldn't fetch the new listens");

    let user_listens = RecordingWithListens::from_listencollection(conn, user_listens).await?;

    // Get the global listen count
    println_cli("[Seeding] Getting global listen counts");
    let recording_ids = recordings
        .iter_recordings()
        .map(|r| r.mbid.to_string())
        .collect_vec();
    let global_listen_counts = get_global_listen_counts(&recording_ids).await?;

    println_cli("[Sorting] Calculating underated scores");
    let sorted = underrated_sorter(
        recordings.into_values().collect_vec(),
        &user_listens,
        global_listen_counts,
    );

    println_cli("[Finalising] Creating radio playlist");
    let collected = collector
        .collect(stream::iter(sorted).map(|r| r.recording().clone()))
        .await;

    println_cli("[Sending] Sending radio playlist to listenbrainz");
    PlaylistStub::new(
        "Radio: Underrated recordings".to_string(),
        Some(username.to_string()),
        true,
        collected
            .into_iter()
            .map(|r| RecordingMBID::from(r.mbid))
            .collect(),
        Some(
            format!("A playlist containing all the tracks that {username} listen to, 
        but seemingly no one else does. Come take a listen if you want to find hidden gems!<br>
        <br>
        The mix is made by calculating a score for each listen. This score is composed of two values:<br>
        - The rank in {username}'s top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)<br>
        - The percentage of the recording's listens being from {username} (Made with this formula: (user listens / worldwide listens) *100)<br>
        <br>
        Made with: https://github.com/RustyNova016/Alistral"
        )),
    )
    .send(token)
    .await?;

    Ok(())
}
