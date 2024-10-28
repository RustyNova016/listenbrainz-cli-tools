use chrono::{Duration, Utc};
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use rust_decimal::Decimal;

use crate::database::get_db_client;
use crate::database::listenbrainz::listens::fetch_latest_listens_of_user;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::radio::RadioConfig;
use crate::utils::playlist::PlaylistStub;

pub async fn overdue_radio(
    username: &str,
    token: &str,
    min_listens: Option<u64>,
    cooldown: u64,
    overdue_factor: bool,
    config: RadioConfig,
) -> color_eyre::Result<()> {
    let db = get_db_client().await;
    let conn = &mut *db.connection.acquire().await?;
    let deadline = Utc::now() - Duration::hours(cooldown as i64);
    let time_stamp = deadline.timestamp();

    // Get the listens
    fetch_latest_listens_of_user(get_db_client().await.as_welds_client(), username).await?;
    println!("Getting...");
    let listens: ListenCollection = sqlx::query_as!(
        Listen,
        "
        SELECT 
            listens.*
        FROM       
            users 
            INNER JOIN listens ON users.name = listens.user 
            INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
        WHERE
            -- Only for this user
            LOWER(listens.user) = LOWER(?)  

            -- Keep only mapped listens 
            AND msid_mapping.user = users.id 

            -- Give some recordings a cooldown period 
            AND listened_at < ?
        ORDER BY msid_mapping.recording_mbid",
        username,
        time_stamp
    )
    .fetch_all(&mut *conn)
    .await?
    .into();

    // Now let's group them by Recording ID
    println!("Grouping...");
    let mut recordings = RecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();

    recordings.retain(|data| data.len() as u64 > min_listens.unwrap_or(2_u64));

    // Sort
    let scores = if !overdue_factor {
        recordings
            .into_iter()
            .map(|r| {
                (
                    Decimal::from(r.overdue_by().num_seconds()),
                    r.recording().clone(),
                )
            })
            .collect_vec()
    } else {
        recordings
            .into_iter()
            .map(|r| (r.overdue_factor(), r.recording().clone()))
            .collect_vec()
    };

    let sorted_scores = RadioConfig::sort_scores2(scores);

    let playlist = config
        .finalize_radio_playlist_from_vec::<()>(sorted_scores)
        .await
        .unwrap();

    PlaylistStub::new(
        "Radio: Overdue listens".to_string(),
        Some(username.to_string()),
        true,
        playlist
            .into_iter()
            .map(|r| RecordingMBID::from(r.mbid))
            .collect(),
        Some(
            "Automatically generated by: https://github.com/RustyNova016/listenbrainz-cli-tools"
                .to_string(),
        ),
    )
    .send(token)
    .await?;

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn overdue_by() {
    use crate::models::radio::RadioConfigBuilder;
    let var_name = RadioConfigBuilder::default();
    overdue_radio("RustyNova", "t", None, 0, false, var_name.build().unwrap())
        .await
        .unwrap();
}
