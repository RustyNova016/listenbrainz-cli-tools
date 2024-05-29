use itertools::Itertools;
use crate::core::entity_traits::mbid::{IsMbid, VecIExt};
use crate::models::clippy::clippy_error::missing_work::MissingWorkError;
use crate::models::clippy::clippy_error::IsClippyError;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::utils::cli_paging::CLIPager;
use crate::utils::println_cli;

pub async fn clippy_tool() {
    let user_listens = UserListens::get_user_with_refresh("rustynova")
    .await
    .expect("Couldn't fetch the new listens");

    println_cli("Running clippy...");
    println!();

    let mut pager = CLIPager::new(1);
    let mut checked_ids =  user_listens.get_mapped_listens().get_listened_recordings_mbids().await.unwrap().into_iter().map(|id| id.into_mbid()).collect_vec();

    while let Some(current_id) = checked_ids.pop() {
        MUSICBRAINZ_DATABASE.remove(&current_id.clone().into_mbid()).await.unwrap();
        add_relations_to_queue(&mut checked_ids, &current_id).await.unwrap();
        checked_ids = checked_ids.into_iter().unique().collect_vec();
        
        let has_error = clippy_mbid(current_id).await.unwrap();
        if has_error {
            if !pager.count_once() {
                break;
            }
        }
    }
}

async fn clippy_mbid(id: MBID) -> color_eyre::Result<bool> {
    let mut error_count = 0;

    if MissingWorkError::check_and_print(id).await? {error_count += 1}

    Ok(error_count > 0)
}

async fn add_relations_to_queue(queue: &mut Vec<MBID>, id: &MBID) -> color_eyre::Result<()> {
    match id {
        MBID::Recording(id) => {
            let recording = id.get_or_fetch_entity().await?;
            
            queue.extend(recording.get_credited_artists_ids().unwrap_or_else(Vec::new).into_mbids());
            queue.extend(recording.releases().as_ref().cloned().unwrap_or_else(Vec::new).into_mbids());
        }
        _ => {}
    }
    
    Ok(())
}