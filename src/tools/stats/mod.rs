use crate::models::api::listenbrainz::ListenBrainzAPI;
use crate::models::data::recording::Artist;
use crate::models::stats::recording_stats::RecordingStatsSorter;
use crate::utils::traits::VecWrapper;
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
    sorter.extend(user_listens.get_mapped_listens()).expect("Couldn't sort the listens");

    let mut pager = CLIPager::new(5);
    for (_key, listens) in sorter.into_sorted() {
        let pager_continue = pager.execute(|| {
            println!(
                "[{}] {} - {}",
                listens.len(),
                listens
                    .first()
                    .unwrap()
                    .get_mapping_data()
                    .as_ref()
                    .unwrap()
                    .get_recording_name(),
                listens
                    .first()
                    .unwrap()
                    .mapping_data
                    .as_ref()
                    .unwrap()
                    .artist_credit
                    .as_ref()
                    .unwrap_or(&"".to_string())
            )
        });

        if !pager_continue {
            return;
        };
    }
}

pub fn stats_artist(username: &str) {
    // Get the listens
    let mut lb_api = ListenBrainzAPI::new();
    let user_listens = lb_api
        .fetch_listens_of_user_cached(username)
        .expect("Couldn't fetch the new listens");

    // Data sorting
    let mut sorter = ArtistStatsSorter::new();
    sorter.extend(user_listens.get_mapped_listens()).expect("Couldn't sort the listens");

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
