use chrono::prelude::Utc;
use chrono::Duration;
use itertools::Itertools;
use listenbrainz::raw::Client;

use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::playlist::PlaylistStub;
use crate::utils::println_cli;

pub async fn overdue_radio(
    username: &str,
    token: &str,
    min_listens: Option<u64>,
    cooldown: u64,
) {
    let mut listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let deadline = Utc::now() - Duration::hours(cooldown as i64);
    let blacklisted_recordings = listens
        .get_listened_after(&deadline)
        .into_iter()
        .map(|listen| {
            listen
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped!")
                .recording_mbid()
                .clone()
        })
        .collect_vec();

    // Filter out all the listens of blacklisted recordings
    listens.retain(|listen| {
        listen.get_mapping_data().as_ref().is_some_and(|mapping| {
            !blacklisted_recordings.contains(&mapping.recording_mbid.clone())
        })
    });

    let mut scores = listens
        .get_listen_rates()
        .await
        .expect("Couldn't calculate the listens rates");

    // Filter minimum
    scores.retain(|rate| rate.1.listen_count() > &min_listens.unwrap_or(3_u64));

    // Sort
    scores.sort_by_cached_key(|rate| {
        rate.1.get_estimated_date_of_next_listen(&rate.0) - Utc::now()
    });

    let chunked = scores.chunks(50).collect_vec();
    let bests = chunked
        .first()
        .expect("No recordings have been listened to");

    for rate in *bests {
        println_cli(format!(
            "Adding [{}]. \n - Last listen was the: {} \n - Average time between listens: {} \n - Estimated date of next listen: {}",
            Recording::get_cached_or_fetch(rate.1.recording())
                .await
                .unwrap()
                .title,
            rate.0.get_latest_listen().map(|listen| listen.listened_at).unwrap_or(Utc::now()),
            rate.1.get_average_time_between_listens(),
            rate.1.get_estimated_date_of_next_listen(&rate.0)
        ));
    }

    let playlist_stub = PlaylistStub::new(
        "Radio: Overdue listens".to_string(),
        Some(username.to_string()),
        true,
        bests.iter().map(|rate| rate.1.recording().clone().into()).collect_vec(), // TODO: Remove cast to recordingmbid
        Some(format!("A playlist containing all the tracks that {username} listen to, 
            but seemingly no one else does. Come take a listen if you want to find hidden gems!<br>
            <br>
            The mix is made by calculating a score for each listen. This score is composed of two values:<br>
            - The rank in {username}'s top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)<br>
            - The percentage of the recording's listens being from {username} (Made with this formula: (user listens / worldwide listens) *100)<br>
            <br>
            Made with: https://github.com/RustyNova016/listenbrainz-cli-tools"
        )),
    );

    Client::new()
        .playlist_create(token, playlist_stub.into_jspf())
        .expect("Couldn't send the playlist");
}
