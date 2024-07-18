use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::SortSorterBy;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::work::Work;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_works(stats: StatisticSorter, sort_by: SortSorterBy) {
    let mut pager = CLIPager::new(5);

    for (key, data) in stats.into_sorted_vec(sort_by) {
        let work = Work::get_load_or_fetch(&key.clone().into()) //TODO: Use MBIDs directly
            .await
            .expect("Couldn't get work data");

        let pager_continue = pager.execute(|| {
            println!("[{}] - {}", data.len(), work.title(),);
        });

        if !pager_continue {
            return;
        };
    }
}
