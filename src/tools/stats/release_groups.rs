use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::SortSorterBy;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_release_groups(stats: StatisticSorter, sort_by: SortSorterBy) {
    let mut pager = CLIPager::new(5);

    for (key, data) in stats.into_sorted_vec(sort_by) {
        let release_group = ReleaseGroup::get_cached_or_fetch(&key).await.unwrap();

        let artist_credit_string = release_group
            .get_or_fetch_artist_credits()
            .await
            .unwrap()
            .get_artist_credit_as_string();
        let pager_continue = pager.execute(|| {
            println!(
                "[{}] - {} by {}",
                data.len(),
                release_group.title(),
                artist_credit_string
            );
        });

        if !pager_continue {
            return;
        };
    }
}
