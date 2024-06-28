use core::num;
use std::ops::Div;

use itertools::min;
use itertools::Itertools;
use rust_decimal::Decimal;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::recording_with_listens::recording_id::RecordingIDWithListens;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

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
    let a_percent = Decimal::new(recording_shared.len().try_into().unwrap(), 0) / Decimal::new(unique_recordings_ids_a.len().try_into().unwrap(), 0) * Decimal::ONE_HUNDRED;
    println!("This is {}% of {}'s listened recordings", a_percent, user_a);
    let b_percent = Decimal::new(recording_shared.len().try_into().unwrap(), 0) / Decimal::new(unique_recordings_ids_b.len().try_into().unwrap(), 0) * Decimal::ONE_HUNDRED;
    println!("This is {}% of {}'s listened recordings", b_percent, user_b);

    let mapped_rec_a = RecordingIDWithListens::all_from_unfiltered(&listens_user_a).await?;

    let mapped_rec_b = RecordingIDWithListens::all_from_unfiltered(&listens_user_b).await?;

    

    Ok(())
}

async fn calculate_listen_ratio(user_listens: &ListenCollection) -> color_eyre::Result<Vec<(Decimal, RecordingMBID)>> {
    let mapped_listens = RecordingIDWithListens::all_from_unfiltered(&user_listens).await?;
    let num_total_listens = Decimal::new(user_listens.len().try_into().unwrap(), 0);

    Ok(mapped_listens.into_iter().map(|mapped| { 
            (Decimal::new(mapped.listen_count().try_into().unwrap(), 0) /num_total_listens, mapped.recording_id().clone()) 
        }).collect_vec())
}

fn compare_ratios(ratios_a: Vec<(Decimal, RecordingMBID)>, ratios_b: Vec<(Decimal, RecordingMBID)>, shared_recordings: Vec<RecordingMBID>) {
    let mut total_ratio = Decimal::ZERO;

    for rec in shared_recordings {
        let Some(ratio_a) = ratios_a.iter().find(|(_, id)| id == &rec) else {continue};
        let Some(ratio_b) = ratios_b.iter().find(|(_, id)| id == &rec) else {continue};

        if ratio_a < ratio_b {
            
        }
    }
}