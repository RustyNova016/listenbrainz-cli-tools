use color_eyre::owo_colors::OwoColorize;

use crate::{
    models::{
        api::fetch_listens,
        cli::stats::GroupByTarget,
        data::listens::{collection::UserListenCollection, UserListen},
        stats::{
            artist_stats::{ArtistStats, ArtistStatsSorter},
            stat_struct::EntityStats,
            StatSorter,
        },
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
    sorter.extend(listens.get_mapped_listens());

    for key in sorter.into_sorted() {
        let mut pager = CLIPager::new(5);

        pager.execute(|| {
            println!(
                "[{}] - {}",
                key.len(),
                key.first()
                    .unwrap()
                    .get_mapping_data()
                    .as_ref()
                    .unwrap()
                    .get_recording_name()
            );
        });
    }
}
