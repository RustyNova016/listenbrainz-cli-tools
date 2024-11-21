use std::fs::File;
use std::path::PathBuf;

use listenbrainz::raw::response::UserListensListen;

use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::utils::println_cli_info;

//TODO: Move this file in the tool folder

pub async fn load_listen_dump(
    path: &PathBuf, 
    username: &str,
) -> Result<(), color_eyre::eyre::Error> {

    Ok(())
}

