//! Contain the code for the "unlisted" command
use color_eyre::eyre::Context;
use listenbrainz::raw::{response::UserListensListen, Client};
use listenbrainz_utils::readers::ListenReaderBuilder;

pub fn unlinked_command(username: &str) {
    println!("Fetching unlinkeds for user {}", username);
    let unlinked = get_all_unlinked_of_user(username);
    for ele in unlinked {
        println!(
            "{:#?} - {} [{}]",
            ele.track_metadata.release_name, ele.track_metadata.artist_name, ele.recording_msid
        )
    }
}

/// Fetch an user's listens and extract the unlinked ones
pub fn get_all_unlinked_of_user(username: &str) -> Vec<UserListensListen> {
    let client = Client::new();

    let mut builder = ListenReaderBuilder::default();
    builder.user_name(username.to_string());
    let mut reader = builder
        .build()
        .context("Couldn't create ListenReader")
        .unwrap();

    let mut unlinked = vec![];
    let mut i = 1;
    loop {
        println!("Page: {}", i);
        i += 1;
        let page = reader.next(&client).unwrap();

        for listen in page.payload.listens.into_iter() {
            if listen.track_metadata.mbid_mapping.is_none() {
                unlinked.push(listen);
            }
        }

        if page.payload.count <= 1 {
            break;
        }
    }
    unlinked
}
