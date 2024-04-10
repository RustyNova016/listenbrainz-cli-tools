use indicatif::ProgressBar;

use crate::models::cache::global_cache::GlobalCache;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::stats::artist_stats::ArtistStatsSorter;
use crate::models::stats::StatSorter;
use crate::utils::cli_paging::CLIPager;
use crate::utils::Logger;

pub async fn stats_artist(username: &str) {
    // Get the listens
    let user_listens = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens")
        .expect("Couldn't fetch the new listens");

    let mapped_listens = user_listens.get_mapped_listens();

    let progress_bar = ProgressBar::new(mapped_listens.len().try_into().unwrap());
    Logger::set_global_overide(progress_bar.clone());

    // Data sorting
    let mut sorter = ArtistStatsSorter::new();
    sorter
        .extend(progress_bar.wrap_iter(mapped_listens.into_iter()))
        .await
        .expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);
    for (key, data) in sorter.into_sorted() {
        let artist = Artist::get_or_fetch(&key).await.unwrap();

        let pager_continue = pager.execute(|| {
            println!("[{}] - {}", data.len(), artist.name);
        });

        if !pager_continue {
            return;
        };
    }
}
