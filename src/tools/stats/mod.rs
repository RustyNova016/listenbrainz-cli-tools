use color_eyre::owo_colors::OwoColorize;

use crate::models::{api::fetch_listens, cli::stats::GroupByTarget, musicbrainz::MBIDType, stats::{stat_struct::EntityStats, MBIDStatCounter}};

pub fn stats_command(username: &str, target: GroupByTarget) {
    println!("{} Getting the listens...", "[CLI Tools]".green());
    let listens = fetch_listens(username).unwrap();
    println!("{} Calculating stats...", "[CLI Tools]".green());
    let mut sorter = EntityStats::new(target);
    sorter.extend(listens.get_mapped_listens());


    match target {
        GroupByTarget::Recording => {print_recording_stats(sorter);}
        GroupByTarget::Artist => {print_artist_stats(sorter);}
    }
    
}

pub fn print_recording_stats(sorter: EntityStats) {
    for key in sorter.into_sorted() {
        println!(
            "[{}] - {}",
            key.len(),
            key.first()
                .unwrap()
                .get_mapping_data()
                .as_ref()
                .unwrap()
                .get_recording_name()
        )
    }
}

pub fn print_artist_stats(sorter: EntityStats) {
    for key in sorter.into_sorted() {
        println!(
            "[{}] - {}",
            key.len(),
            key.first()
                .unwrap()
                .get_mapping_data()
                .as_ref()
                .unwrap()
                .get_recording_name()
        )
    }
}
