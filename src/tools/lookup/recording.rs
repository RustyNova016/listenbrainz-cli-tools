use chrono::Duration;
use chrono::Local;
use humantime::format_duration;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::database::get_db_client;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::recording_with_listens::RecordingWithListens;
use crate::models::config::Config;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::cli::await_next;
use crate::utils::extensions::chrono_ext::DateTimeUtcExt;
use crate::utils::extensions::chrono_ext::DurationExt;
use crate::utils::println_cli;

pub async fn lookup_recording(username: &str, id: RecordingMBID) -> color_eyre::Result<()> {
    let db = get_db_client().await;
    let conn = &mut *db.as_sqlx_pool().acquire().await?;

    // Prefetch the listens. TODO: Merge specific rrecording fetching into query
    ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::None)
        .user(username.to_string())
        .build()
        .fetch(&db)
        .await?;

    let Some(recording) = Recording::get_or_fetch(conn, &id.to_string()).await? else {
        println_cli(format!("Couldn't find the recording with id: {}", id));
        return Ok(());
    };

    let listens = Listen::get_listens_of_recording_by_user(conn, username, recording.id).await?;
    let grouped = ListenCollection::new(listens)
        .group_by_recording(conn)
        .await?;
    let coupled = RecordingWithListens::from_group_by(grouped)
        .await?
        .pop()
        .unwrap(); //TODO: Handle unlistened

    lookup_recording_listened(coupled, conn).await?;

    //if recording_info.is_listened() {
    //    lookup_recording_listened(recording_info).await?;
    //} else {
    //    lookup_recording_unlistened(recording_info).await?;
    //}

    await_next();

    Ok(())
}

// async fn lookup_recording_unlistened(
//     recording_info: RecordingWithListens,
// ) -> color_eyre::Result<()> {
//     let data_string = format!(
//         "\nHere are the statistics of {} ({})

//         The recording hasn't been listened to yet",
//         recording_info.recording().get_title_with_credits().await?,
//         recording_info.recording_id()
//     );

//     println_cli(data_string);
//     Ok(())
// }

async fn lookup_recording_listened(
    recording_info: RecordingWithListens,
    conn: &mut sqlx::SqliteConnection,
) -> color_eyre::Result<()> {
    let conf = Config::load_or_panic();
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
        \n    - Overdue score: {}\
        \n    - Overdue score (with multiplier): {}\
        \n", // \n    - Underated score: {}\
        recording_info.recording().format_with_credits(conn).await?,
        recording_info.recording().mbid,
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
        //recording_info
        //    .underated_score_single()
        //    .await?
        //    .trunc_with_scale(2),
        recording_info.overdue_factor().trunc_with_scale(2),
        (recording_info.overdue_factor() * conf.bumps.get_multiplier(&RecordingMBID::from(recording_info.recording().mbid.clone())))
            .trunc_with_scale(2)
    );

    println!("{}", data_string);
    Ok(())
}

fn get_overdue_line(recording_info: &RecordingWithListens) -> String {
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
