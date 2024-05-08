//! Contain the code for the "unlisted" command
use std::cmp::Reverse;

use color_eyre::eyre::Context;
use listenbrainz::raw::response::UserListensListen;
use listenbrainz::raw::Client;

use crate::models::cli::common::SortSorterBy;
use crate::models::data::listenbrainz::messy_recording::MessyRecording;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::utils::cli_paging::CLIPager;
use crate::utils::{println_cli, ListenAPIPaginatorBuilder};

pub async fn unmapped_command(username: &str, sort: Option<SortSorterBy>) {
    println_cli(format!("Fetching unmapped for user {}", username));
    let unlinked = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_unmapped_listens();

    let mut messy_recordings: Vec<MessyRecording> = vec![];
    let unlinked_count = unlinked.len();

    // We put all the listens in a MessyBrain recordings
    for listen in unlinked {
        let messy_recording = messy_recordings
            .iter_mut()
            .find(|record| record.id == listen.messybrainz_data.msid);

        if let Some(messy_recording) = messy_recording {
            messy_recording.add_listen(listen);
        } else {
            let mut messy_recording = MessyRecording::new(listen.messybrainz_data.msid.clone());
            messy_recording.add_listen(listen);
            messy_recordings.push(messy_recording);
        }
    }

    match sort.unwrap_or_default() {
        SortSorterBy::Name => {
            messy_recordings.sort_by_key(|recording| recording.get_recording_name());
        }

        SortSorterBy::Oldest => {
            messy_recordings.sort_by_key(|recording| {
                recording
                    .get_oldest_listen()
                    .map(|listen| listen.listened_at)
            });
        }

        SortSorterBy::Count => {
            messy_recordings.sort_by_key(|recording| Reverse(recording.associated_listens.len()));
        }
    }

    println!("Done! Here are {}'s top unmapped listens:", username);

    let mut pager = CLIPager::new(5);

    println!("Total: {} unmapped recordings", unlinked_count);
    for record in messy_recordings.iter() {
        let pager_continue = pager.execute(|| {
            println!(
                "({}) {} - {}",
                record.associated_listens.len(),
                record.get_recording_name().unwrap_or_default(),
                record.get_artist_name().unwrap_or_default()
            );

            let latest_listen = record.get_latest_listen();

            println!(
                "    -> https://listenbrainz.org/user/{}/?min_ts={}&max_ts={}",
                username,
                latest_listen
                    .map(|listen| listen.listened_at.timestamp() - 1)
                    .unwrap_or(0),
                latest_listen
                    .map(|listen| listen.listened_at.timestamp() + 1)
                    .unwrap_or(0)
            );
        });

        if !pager_continue {
            return;
        }
    }
}

/// Fetch an user's listens and extract the unlinked ones
pub fn get_all_unlinked_of_user(username: &str) -> Vec<UserListensListen> {
    let client = Client::new();

    let mut builder = ListenAPIPaginatorBuilder::default();
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
