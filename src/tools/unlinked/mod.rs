//! Contain the code for the "unlisted" command
use color_eyre::eyre::Context;
use listenbrainz::raw::{response::UserListensListen, Client};
use listenbrainz_utils::readers::ListenReaderBuilder;

use crate::models::messy_recording::{self, MessyRecording};

pub fn unlinked_command(username: &str) {
    println!("Fetching unlinkeds for user {}", username);
    let unlinked = get_all_unlinked_of_user(username);

    let mut messy_recordings: Vec<MessyRecording> = vec![];

    // We put all the listens in a MessyBrain recordings
    for listen in unlinked {
        println!(
            "{:#?} - {} [{}]",
            listen.track_metadata.release_name,
            listen.track_metadata.artist_name,
            listen.recording_msid
        );

        let mut messy_recording = messy_recordings
            .iter_mut()
            .find(|record| record.id == listen.recording_msid);

        if let Some(messy_recording) = messy_recording {
            messy_recording.add_listen(listen)
        } else {
            let mut messy_recording = MessyRecording::new(listen.recording_msid.clone());
            messy_recording.add_listen(listen);
            messy_recordings.push(messy_recording)
        }
    }

    messy_recordings.sort_by_key(|recording| recording.associated_listens.len());

    messy_recordings.iter().for_each(|record| {
        println!(
            "({}) {} - {}",
            record.associated_listens.len(),
            record.get_recording_name().unwrap_or_default(),
            record.get_artist_name().unwrap_or_default()
        )
    })
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
