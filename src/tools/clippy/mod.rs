use crate::core::entity_traits::mbid::IsMbid;
use crate::models::clippy::clippy_error::missing_work::MissingWorkError;
use crate::models::clippy::clippy_error::IsClippyError;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::utils::cli_paging::CLIPager;
use crate::utils::println_cli;

pub async fn clippy_tool() {
    let user_listens = UserListens::get_user_with_refresh("rustynova")
    .await
    .expect("Couldn't fetch the new listens");

    println_cli("Running clippy...");
    println!();

    let mut pager = CLIPager::new(1);

    for recording in user_listens.get_mapped_listens().get_listened_recordings_mbids().await.unwrap() {
        if clippy_mbid(recording.into_mbid()).await.unwrap() {
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