use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::models::cli::common::SortSorterBy;
use crate::models::cli::common::StatsTarget;

mod artists;
mod recordings;
mod release_groups;
mod releases;
mod work;

pub async fn stats_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    target: StatsTarget,
    _sort_by: SortSorterBy,
) {
    let listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await
        .expect("Couldn't fetch the new listens");

    match target {
        StatsTarget::Recording => {
            recordings::stats_recording(conn, listens).await;
        }
        StatsTarget::RecordingTime => {
            recordings::stats_recording_time(conn, listens).await;
        }
        StatsTarget::Artist => {
            artists::stats_artist(conn, listens).await;
        }
        StatsTarget::Release => {
            releases::stats_releases(conn, listens).await;
        }
        StatsTarget::ReleaseGroup => {
            release_groups::stats_release_groups(conn, listens).await;
        }
        StatsTarget::Work => {
            work::stats_works(conn, listens).await;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     // use crate::utils::println_cli_info;

//     // use super::*;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn stats_command_recordings() {
//         //let mut clog = colog::default_builder();
//         //clog.filter(None, log::LevelFilter::Trace);
//         //clog.init();

//         // println_cli_info("--- Starting test ---");
//         // stats_command("RustyNova", GroupByTarget::Recording, SortSorterBy::Count).await;
//     }
// }
