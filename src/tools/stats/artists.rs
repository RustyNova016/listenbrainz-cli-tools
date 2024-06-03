use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::SortSorterBy;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_artist(stats: StatisticSorter, sort_by: SortSorterBy) {
    let mut pager = CLIPager::new(5);

    for (key, data) in stats.into_sorted_vec(sort_by) {
        let artist = Artist::get_cached_or_fetch(&key.clone().into())
            .await
            .unwrap(); // TODO: Use MBIDs

        let pager_continue = pager.execute(|| {
            println!("[{}] - {}", data.len(), artist.name());
        });

        if !pager_continue {
            return;
        };
    }
}
