use core::cmp::Reverse;

use itertools::Itertools;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_recording(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = RecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.len()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        println!(
            "[{}] {}",
            group.len(),
            group
                .recording()
                .format_with_credits(conn)
                .await
                .expect("Error getting formated recording name"),
        );

        if !pager.inc() {
            break;
        }
    }
}
