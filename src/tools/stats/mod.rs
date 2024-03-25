use color_eyre::owo_colors::OwoColorize;

use crate::models::api::musicbrainz::MusicBrainzAPI;
use crate::{
    models::{
        api::fetch_listens,
        cli::stats::GroupByTarget,
        data::listens::collection::UserListenCollection,
        stats::{artist_stats::ArtistStatsSorter, StatSorter},
    },
    utils::cli_paging::CLIPager,
};

pub fn stats_command(username: &str, target: GroupByTarget) {
    println!("{} Getting the listens...", "[CLI Tools]".green());
    let listens = fetch_listens(username).unwrap();
    println!("{} Calculating stats...", "[CLI Tools]".green());

    match target {
        GroupByTarget::Recording => {
            //stats_recording(listens);
        }
        GroupByTarget::Artist => {
            stats_artist(listens);
        }
    }
}

//pub fn stats_recording(listens: UserListenCollection) {
//    let mut sorter = EntityStats::new(target);
//    sorter.extend(listens.get_mapped_listens());
//
//    for key in sorter.into_sorted() {
//        println!(
//            "[{}] - {}",
//            key.len(),
//            key.first()
//                .unwrap()
//                .get_mapping_data()
//                .as_ref()
//                .unwrap()
//                .get_recording_name()
//        )
//    }
//}

pub fn stats_artist(listens: UserListenCollection) {
    let mut sorter = ArtistStatsSorter::new();
    let mut mb_api = MusicBrainzAPI::new();
    mb_api.save_cache();

    sorter.extend(listens.get_mapped_listens(), &mut mb_api);

    mb_api.save_cache();

    let mut pager = CLIPager::new(5);
    for (key, data) in sorter.into_sorted() {
        let artist = mb_api.get_artist(key.clone());

        if !pager.execute(|| {
            println!("[{}] - {}", data.len(), artist.name);
        }) {
            return;
        };
    }
}
