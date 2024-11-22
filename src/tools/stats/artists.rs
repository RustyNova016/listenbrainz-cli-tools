use core::cmp::Reverse;

use itertools::Itertools;

use crate::database::get_conn;
use crate::datastructures::entity_with_listens::artist_with_listens::ArtistWithListens;
use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_artist(listens: ListenCollection) {
    let mut groups = ArtistWithListens::from_listencollection(&mut *get_conn().await, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        group
            .artist()
            .fetch_if_incomplete(&mut *get_conn().await)
            .await
            .expect("Error while fetching release");
        println!("[{}] {}", group.listen_count(), group.artist().name);

        if !pager.inc() {
            break;
        }
    }
}
