use crate::models::cache::global_cache::GlobalCache;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::stats::artist_stats::ArtistStatsSorter;
use crate::models::stats::StatSorter;
use crate::utils::cli_paging::CLIPager;

pub fn stats_artist(username: &str) {
    // Get the listens
    let user_listens = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens")
        .expect("Couldn't fetch the new listens");

    // Data sorting
    let mut sorter = ArtistStatsSorter::new();
    sorter
        .extend(user_listens.get_mapped_listens())
        .expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);
    for (key, data) in sorter.into_sorted() {
        let artist = Artist::get_or_fetch(&key).unwrap();

        let pager_continue = pager.execute(|| {
            println!("[{}] - {}", data.len(), artist.name);
        });

        if !pager_continue {
            return;
        };
    }
}
