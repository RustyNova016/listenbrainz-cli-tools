use chrono::Duration;
use chrono::Local;
use humantime::format_duration;

use crate::models::data::listenbrainz::listens_with_entity::ListensWithEntity;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::cli::await_next;
use crate::utils::extensions::chrono_ext::DateTimeUtcExt;
use crate::utils::extensions::chrono_ext::DurationExt;
use crate::utils::println_cli;

pub async fn lookup_recording(username: &str, id: NaiveMBID<Recording>) -> color_eyre::Result<()> {
    let listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_primary_listens()
        .await?;

    let recording_info =
        ListensWithEntity::<Recording>::from_mapping(id.get_load_or_fetch().await?, &listens);

    if recording_info.is_listened() {
        lookup_recording_listened(recording_info).await?;
    } else {
        lookup_recording_unlistened(recording_info).await?;
    }

    await_next();

    Ok(())
}

async fn lookup_recording_unlistened(
    recording_info: ListensWithEntity<Recording>,
) -> color_eyre::Result<()> {
    let data_string = format!(
        "\nHere are the statistics of {} ({})
        
        The recording hasn't been listened to yet",
        recording_info.entity().get_title_with_credits().await?,
        recording_info.entity().get_mbid()
    );

    println_cli(data_string);
    Ok(())
}

async fn lookup_recording_listened(
    recording_info: ListensWithEntity<Recording>,
) -> color_eyre::Result<()> {
    let data_string = format!(
        " ---
        \nHere are the statistics of {} ({})\
        \n\
        \n [General]\
        \n    - Listen count: {}\
        \n    - First listened: {}\
        \n    - Last listened: {}
        
        \n [Listening rate]\
        \n    - Average duration between listens: {}\
        \n    - Estimated date of next listen: {}\
        {}

        \n [Radios]\
        \n    - Underated score: {}\
        \n    - Overdue score: {}\
        \n",
        recording_info.entity().get_title_with_credits().await?,
        recording_info.entity().get_mbid(),
        recording_info.listen_count(),
        recording_info
            .first_listen_date()
            .unwrap()
            .floor_to_second()
            .with_timezone(&Local),
        recording_info
            .last_listen_date()
            .unwrap()
            .floor_to_second()
            .with_timezone(&Local),
        format_duration(
            recording_info
                .average_duration_between_listens()
                .floor_to_minute()
                .to_std()
                .unwrap()
        ),
        recording_info
            .estimated_date_of_next_listen()
            .unwrap()
            .floor_to_second()
            .with_timezone(&Local),
        get_overdue_line(&recording_info),
        recording_info
            .underated_score_single()
            .await?
            .trunc_with_scale(2),
        recording_info.overdue_score().trunc_with_scale(2),
    );

    println_cli(data_string);
    Ok(())
}

fn get_overdue_line(recording_info: &ListensWithEntity<Recording>) -> String {
    let time = recording_info.overdue_by();

    if time <= Duration::zero() {
        return String::new();
    }

    println!("{}", time.num_minutes());

    format!(
        "\n    - Overdue by: {}",
        format_duration(time.floor_to_minute().to_std().unwrap())
    )
}
