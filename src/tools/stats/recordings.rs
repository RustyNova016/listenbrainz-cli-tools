use core::cmp::Reverse;

use itertools::Itertools;

use crate::database::get_conn;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_recording(listens: ListenCollection) {
    let mut groups = RecordingWithListens::from_listencollection(&mut *get_conn().await, listens)
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
                .format_with_credits(&mut *get_conn().await)
                .await
                .expect("Error getting formated recording name"),
        );

        if !pager.inc() {
            break;
        }
    }
}
