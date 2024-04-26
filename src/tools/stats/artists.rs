use std::sync::Arc;

use crate::core::caching::global_cache::GlobalCache;
use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use indicatif::ProgressBar;

use crate::models::data::musicbrainz::artist::Artist;
use crate::models::stats::artist_stats::ArtistStatisticSorter;

use crate::core::statistics::statistic_holder::StatisticHolder;
use crate::utils::cli_paging::CLIPager;
use crate::utils::Logger;

pub async fn stats_artist(username: &str) {
    // Get the listens
    let user_listens = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens");

    let mapped_listens = user_listens.get_mapped_listens();

    let progress_bar = ProgressBar::new(mapped_listens.len().try_into().unwrap());
    Logger::set_global_overide(progress_bar.clone());

    // Data sorting
    let sorter = Arc::new(ArtistStatisticSorter::new());

    // let mut tasks = Vec::new();

    // for listen in mapped_listens.into_iter() {
    //     let sorter_clone = sorter.clone();
    //     tasks.push(tokio::spawn(async move {sorter_clone.insert_listen(listen).await} ));
    // }

    // for task in tasks {
    //     task.await.expect("Couldn't sort the listens").expect("Couldn't sort the listens");
    // }

     sorter
         .extend(progress_bar.wrap_iter(mapped_listens.into_iter()))
         .await
         .expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);
    let extracted_sorter: ArtistStatisticSorter = sorter.as_ref().clone();
    for (key, data) in extracted_sorter.into_sorted() {
        let artist = Artist::get_cached_or_fetch(&key).await.unwrap();

        let pager_continue = pager.execute(|| {
            println!("[{}] - {}", data.count(), artist.name);
        });

        if !pager_continue {
            return;
        };
    }
}
