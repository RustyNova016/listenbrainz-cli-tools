use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::SortSorterBy;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_recording(stats: StatisticSorter, sort_by: SortSorterBy) {
    let mut pager = CLIPager::new(5);

    for (_key, listens) in stats.into_sorted_vec(sort_by) {
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
                    .recording_name(),
                listens
                    .first()
                    .unwrap()
                    .mapping_data
                    .as_ref()
                    .unwrap()
                    .artist_credit
                    .as_ref()
                    .unwrap_or(&String::new())
            );
        });

        if !pager_continue {
            return;
        };
    }
}
