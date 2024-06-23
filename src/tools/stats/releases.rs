use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::SortSorterBy;
use crate::models::data::musicbrainz::release::Release;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_releases(stats: StatisticSorter, sort_by: SortSorterBy) {
    let mut pager = CLIPager::new(5);

    for (key, data) in stats.into_sorted_vec(sort_by) {
        let release = Release::get_cached_or_fetch(&key.clone().into())
            .await
            .unwrap(); // TODO: Use MBIDs

        let artist_credit_string = release
            .get_or_fetch_artist_credits()
            .await
            .unwrap()
            .get_artist_credit_as_string();
        let pager_continue = pager.execute(|| {
            println!(
                "[{}] - {} by {}",
                data.len(),
                release.title(),
                artist_credit_string
            );
        });

        if !pager_continue {
            return;
        };
    }
}
