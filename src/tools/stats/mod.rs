use crate::models::api::listenbrainz::ListenBrainzAPI;
use crate::models::api::musicbrainz::MusicBrainzAPI;
use crate::models::stats::recording_stats::RecordingStatsSorter;
use crate::{
    models::{
        cli::stats::GroupByTarget,
        stats::{artist_stats::ArtistStatsSorter, StatSorter},
    },
    utils::cli_paging::CLIPager,
};

pub fn stats_command(username: &str, target: GroupByTarget) {
    match target {
        GroupByTarget::Recording => {
            stats_recording(username);
        }
        GroupByTarget::Artist => {
            stats_artist(username);
        }
    }
}

pub fn stats_recording(username: &str) {
    // Get the listens
    let mut lb_api = ListenBrainzAPI::new();
    let user_listens = lb_api
        .fetch_listens_of_user_cached(username)
        .expect("Couldn't fetch the new listens");

    // Data sorting
    let mut sorter = RecordingStatsSorter::new();
    sorter.extend(user_listens.get_mapped_listens(), &mut mb_api);

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

pub fn stats_artist(username: &str) {
    let mut mb_api = MusicBrainzAPI::new();

    // Get the listens
    let mut lb_api = ListenBrainzAPI::new();
    let user_listens = lb_api
        .fetch_listens_of_user_cached(username)
        .expect("Couldn't fetch the new listens");

    // Data sorting
    let mut sorter = ArtistStatsSorter::new();
    sorter.extend(user_listens.get_mapped_listens(), &mut mb_api);

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

    mb_api.save_cache();
}
