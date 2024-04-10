use crate::models::cache::global_cache::GlobalCache;
use crate::models::stats::recording_stats::RecordingStatsSorter;
use crate::models::stats::StatSorter;
use crate::utils::cli_paging::CLIPager;

pub fn stats_recording(username: &str) {
    // Get the listens
    let user_listens = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens")
        .expect("Couldn't fetch the new listens");

    // Data sorting
    let mut sorter = RecordingStatsSorter::new();
    sorter
        .extend(user_listens.get_mapped_listens())
        .expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);
    for (_key, listens) in sorter.into_sorted() {
        let pager_continue = pager.execute(|| {
            println!(
                "[{}] {} - {}",
                listens.len(),
                listens
                    .first()
                    .unwrap()
                    .get_mapping_data()
                    .as_ref()
                    .unwrap()
                    .get_recording_name(),
                listens
                    .first()
                    .unwrap()
                    .mapping_data
                    .as_ref()
                    .unwrap()
                    .artist_credit
                    .as_ref()
                    .unwrap_or(&"".to_string())
            )
        });

        if !pager_continue {
            return;
        };
    }
}
