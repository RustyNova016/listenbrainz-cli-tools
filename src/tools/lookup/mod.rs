use std::time::Duration;

use chrono::Utc;
use humantime::format_duration;

use crate::core::entity_traits::mbid::IsMbid;
use crate::core::statistics::listen_rate::ListenRate;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::println_cli;

pub async fn lookup(username: &str, id: RecordingMBID) {
    let listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let recording_listens = listens.get_listens_of_recording(&id);

    let recording = id
        .get_or_fetch_entity()
        .await
        .expect("Couldn't fetch recording");
    let recording_title = recording.title().clone();

    let artist_credits = recording
        .clone()
        .get_or_fetch_artist_credits()
        .await
        .unwrap();
    let recording_artist = artist_credits
        .first()
        .map(|artist| artist.name().clone())
        .unwrap_or_else(|| "[unknown]".to_string()); // TODO: Properly get string

    let listen_count = recording_listens.len();

    let first_listen_date = *recording_listens
        .get_oldest_listen()
        .unwrap()
        .get_listened_at();
    let last_listen_date = *recording_listens
        .get_latest_listen()
        .unwrap()
        .get_listened_at();

    let listen_rate = ListenRate::new(
        id.clone(),
        listen_count as u64,
        Utc::now() - first_listen_date,
    );
    let average_duration_between_listens: Duration = listen_rate
        .get_average_time_between_listens()
        .to_std()
        .unwrap();
    let average_duration_between_listens_fmt = format_duration(average_duration_between_listens);

    let next_listen_date = listen_rate.get_estimated_date_of_next_listen(&recording_listens);

    println_cli(format!(
        "Here are the statistics of {recording_title} by {recording_artist} ({id}):
    - Listen count: {listen_count}
    - First listened at: {first_listen_date}
    - Last listened at: {last_listen_date},
    - Average days between listens: {average_duration_between_listens_fmt}
    - Estimated date of next listen: {next_listen_date}
    "
    ));
}
