use std::cmp::Reverse;

use chrono::prelude::Utc;
use chrono::Duration;
use humantime::format_duration;
use itertools::Itertools;
use listenbrainz::raw::Client;
use rust_decimal::prelude::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::core::entity_traits::mb_cached::MBCached;
use crate::models::cli::config::SelfEditContext;
use crate::models::config::Config;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::playlist::PlaylistStub;
use crate::utils::println_cli;

pub async fn overdue_radio(
    username: &str,
    token: &str,
    min_listens: Option<u64>,
    cooldown: u64,
    overdue_factor: bool,
) {
    let config = Config::load().unwrap();
    let mut listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens()
        .apply_configuration(&config, &SelfEditContext::RadioSeeding);

    let deadline = Utc::now() - Duration::hours(cooldown as i64);
    let blacklisted_recordings = listens
        .get_listened_after(&deadline)
        .get_listened_recordings_mbids()
        .await
        .unwrap();

    // Filter out all the listens of blacklisted recordings
    listens = listens
        .filter_recordings(&blacklisted_recordings, true, false)
        .await
        .unwrap();

    let mut scores = listens
        .get_listen_rates()
        .await
        .expect("Couldn't calculate the listens rates");

    // Filter minimum
    scores.retain(|rate| rate.1.listen_count() > &min_listens.unwrap_or(3_u64));

    // Sort
    if !overdue_factor {
        scores.sort_by_cached_key(|rate| {
            rate.1.get_estimated_date_of_next_listen(&rate.0) - Utc::now()
        });
    } else {
        scores.sort_by_cached_key(|rate| {
            Reverse(
                Decimal::from_i64(rate.1.get_overdue_by(&rate.0).num_seconds()).unwrap()
                    / Decimal::from_i64(rate.1.get_average_time_between_listens().num_seconds())
                        .unwrap(),
            )
        });
    }

    let chunked = scores.chunks(50).collect_vec();
    let bests = chunked
        .first()
        .expect("No recordings have been listened to");

    for rate in *bests {
        println_cli(format!(
            "Adding [{}] ({}). \n - Last listen was the: {} \n - Average time between listens: {} \n - Estimated date of next listen: {} \n - Time overdue: {} \n - Overdue Factor: {}",
            Recording::get_cached_or_fetch(rate.1.recording())
                .await
                .unwrap()
                .title,
                rate.1.recording(),
            rate.0.get_latest_listen().map(|listen| listen.listened_at).unwrap_or_else(Utc::now),
            format_duration(rate.1.get_average_time_between_listens().abs().to_std().unwrap()),
            rate.1.get_estimated_date_of_next_listen(&rate.0),
            format_duration(rate.1.get_overdue_by(&rate.0).abs().to_std().unwrap()),
            rate.1.get_overdue_by(&rate.0).num_seconds() as f64 / rate.1.get_average_time_between_listens().num_seconds() as f64
        ));
    }

    let playlist_stub = PlaylistStub::new(
        "Radio: Overdue listens".to_string(),
        Some(username.to_string()),
        true,
        bests.iter().map(|rate| rate.1.recording().clone()).collect_vec(), // TODO: Remove cast to recordingmbid
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
