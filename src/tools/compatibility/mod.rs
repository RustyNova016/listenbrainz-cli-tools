use itertools::Itertools;

use crate::models::data::listenbrainz::recording_with_listens::recording_id::RecordingIDWithListens;
use crate::models::data::listenbrainz::user_listens::UserListens;

pub async fn compatibility_command(user_a: &str, user_b: &str) -> color_eyre::Result<()> {
    let listens_user_a = UserListens::get_user_with_refresh(user_a)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let listens_user_b = UserListens::get_user_with_refresh(user_b)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let unique_recordings_ids_a = listens_user_a.get_listened_recordings_mbids().await?;        
    let unique_recordings_ids_b = listens_user_b.get_listened_recordings_mbids().await?;

    let recording_shared = unique_recordings_ids_a.clone().into_iter().filter(|rec| unique_recordings_ids_b.contains(rec)).collect_vec();

    println!("You currently both listened to {} same recordings", recording_shared.len());

    let mapped_rec_a = RecordingIDWithListens::all_from_unfiltered(&listens_user_a).await?;
    let mapped_rec_b = RecordingIDWithListens::all_from_unfiltered(&listens_user_b).await?;

    

    Ok(())
}