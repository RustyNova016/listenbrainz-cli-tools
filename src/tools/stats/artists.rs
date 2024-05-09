use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::models::cli::common::SortSorterBy;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_artist(username: &str) {
    // Get the listens
    let user_listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens");

    let stats = user_listens
        .get_listens()
        .get_artist_statistics()
        .await
        .expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);

    for (key, data) in stats.into_sorted_vec(SortSorterBy::Count) {
        let artist = Artist::get_cached_or_fetch(&key).await.unwrap();

        let pager_continue = pager.execute(|| {
            println!("[{}] - {}", data.len(), artist.name);
        });

        if !pager_continue {
            return;
        };
    }
}
