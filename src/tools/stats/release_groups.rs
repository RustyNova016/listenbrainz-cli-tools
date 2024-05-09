use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;

use crate::models::cli::common::SortSorterBy;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::utils::cli_paging::CLIPager;
use crate::utils::println_cli;

pub async fn stats_release_groups(username: &str) {
    // Get the listens
    let user_listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens");

    println_cli(format!(
        "Total number of listens: {}",
        user_listens.get_listens().len()
    ));

    let stats = user_listens
        .get_listens()
        .get_release_group_statistics()
        .await
        .expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);

    for (key, data) in stats.into_sorted_vec(SortSorterBy::Count) {
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
