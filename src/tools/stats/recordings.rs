use crate::core::caching::global_cache::GlobalCache;
use crate::core::statistics::stat_sorter::StatSorter;
use indicatif::ProgressBar;

use crate::models::stats::recording_stats::RecordingStatsSorter;

use crate::utils::cli_paging::CLIPager;
use crate::utils::Logger;

pub async fn stats_recording(username: &str) {
    // Get the listens
    let mapped_listens = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let progress_bar = ProgressBar::new(mapped_listens.len().try_into().unwrap());
    Logger::set_global_overide(progress_bar.clone());

    // Data sorting
    let mut sorter = RecordingStatsSorter::new();
    sorter
        .extend(progress_bar.wrap_iter(mapped_listens.into_iter()))
        .await
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
