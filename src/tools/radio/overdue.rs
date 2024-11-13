use chrono::Duration;
use futures::{stream, StreamExt};

use crate::database::get_db_client;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::radio::collector::RadioCollector;
use crate::datastructures::radio::filters::cooldown::cooldown_filter;
use crate::datastructures::radio::filters::min_listens::min_listen_filter;
use crate::datastructures::radio::filters::timeouts::timeout_filter;
use crate::datastructures::radio::seeders::listens::ListenSeederBuilder;
use crate::datastructures::radio::sorters::overdue::{overdue_factor_sorter, overdue_sorter};
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::playlist::PlaylistStub;
use crate::utils::println_cli;

pub async fn overdue_radio(
    username: &str,
    token: &str,
    min_listens: Option<u64>,
    cooldown: u64,
    overdue_factor: bool,
    collector: RadioCollector,
) -> color_eyre::Result<()> {
    let db = get_db_client().await;
    let conn = &mut *db.connection.acquire().await?;

    println_cli("[Seeding] Getting listens");
    let recordings = ListenSeederBuilder::default()
        .username(username)
        .build()
        .seed(conn)
        .await
        .expect("Couldn't find seed listens");

    println_cli("[Filter] Filtering minimum listen count");
    let recordings = min_listen_filter(stream::iter(recordings), min_listens.unwrap_or(3));

    println_cli("[Filter] Filtering listen cooldown");
    let recordings = cooldown_filter(recordings, Duration::hours(cooldown as i64));

    println_cli("[Filter] Filtering listen timeouts");
    let recordings = timeout_filter(recordings);

    let recordings: Vec<RecordingWithListens> = if !overdue_factor {
        println_cli("[Sorting] Sorting listen by overdue duration");
        overdue_sorter(recordings.collect().await)
    } else {
        println_cli("[Sorting] Sorting listen by overdue factor");
        overdue_factor_sorter(recordings.collect().await)
    };

    println_cli("[Finalising] Creating radio playlist");
    let collected = collector.collect(stream::iter(recordings)).await;

    println_cli("[Sending] Sending radio playlist to listenbrainz");
    PlaylistStub::new(
        "Radio: Overdue listens".to_string(),
        Some(username.to_string()),
        true,
        collected
            .into_iter()
            .map(|r| RecordingMBID::from(r.mbid))
            .collect(),
        Some(
            "Automatically generated by: https://github.com/RustyNova016/listenbrainz-cli-tools"
                .to_string(),
        ),
    )
    .send(token)
    .await?;

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn overdue_by() {
    use crate::datastructures::radio::collector::RadioCollectorBuilder;
    overdue_radio(
        "RustyNova",
        "t",
        None,
        0,
        false,
        RadioCollectorBuilder::default()
            .count_default()
            .duration_default()
            .build(),
    )
    .await
    .unwrap();
}
