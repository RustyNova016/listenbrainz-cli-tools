use itertools::Itertools;
use listenbrainz::raw::Client;

use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::core::statistics::listen_rate::ListenRate;
use crate::core::statistics::listen_rate::ListenRateRange;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::playlist::PlaylistStub;
use crate::utils::println_cli;

pub async fn listen_rate_radio(
    username: &str,
    token: &str,
    min_rate: Option<ListenRate>,
    min_listens: Option<u64>,
) {
    let mut scores = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_listens()
        .get_listen_rates()
        .await
        .expect("Couldn't calculate the listens rates");

    // Filter minimum
    scores.retain(|rate| *rate.listen_count() > min_listens.unwrap_or(3_u64));

    // Filter minimum rate
    if let Some(min_rate) = min_rate {
        scores.retain(|rate| {
            rate.get_listen_rate(ListenRateRange::Year)
                >= min_rate.get_listen_rate(ListenRateRange::Year)
        });
    }

    // Sort
    scores.sort_by_cached_key(|rate| rate.get_listen_rate(ListenRateRange::Year));

    let chunked = scores.chunks(50).collect_vec();
    let bests = chunked
        .first()
        .expect("No recordings have been listened to");

    for rate in *bests {
        println_cli(format!(
            "Adding [{}]. Yearly listens is: {}",
            Recording::get_cached_or_fetch(rate.recording())
                .await
                .unwrap()
                .title,
            rate.get_listen_rate(ListenRateRange::Year)
        ));
    }

    let playlist_stub = PlaylistStub::new(
        "Radio: Listen Rate".to_string(),
        Some(username.to_string()),
        true,
        bests.iter().map(|x| x.recording().clone().into()).collect_vec(), // TODO: Remove cast to recordingmbid
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
