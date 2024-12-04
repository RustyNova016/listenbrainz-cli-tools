use std::cmp::Reverse;

use itertools::Itertools;

use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::messyrecording_with_listens::MessyRecordingWithListens;
use crate::models::cli::common::SortSorterBy;
use crate::utils::cli_paging::CLIPager;
use crate::utils::println_cli;

pub async fn unmapped_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    sort: Option<SortSorterBy>,
) {
    println_cli(format!("Fetching unmapped for user {username}"));
    let listens = ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Unmapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await
        .expect("Couldn't fetch listens");

    let unlinkeds = MessyRecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Couldn't associate the listen to their messybrainz data");
    //let unlinked_count = unlinkeds.listen_count();

    let mut messy_recordings = unlinkeds.values().collect_vec();

    match sort.unwrap_or_default() {
        SortSorterBy::Name => {
            messy_recordings.sort_by_key(|messy_data| &messy_data.messybrainz_data.recording);
        }

        SortSorterBy::Oldest => {
            messy_recordings.sort_by_key(|messy_data| {
                messy_data
                    .get_oldest_listen()
                    .map(|listen| listen.listened_at)
            });
        }

        SortSorterBy::Count => {
            messy_recordings.sort_by_key(|messy_data| Reverse(messy_data.associated_listens.len()));
        }
    }

    println!("Done! Here are {username}'s top unmapped listens:");

    let mut pager = CLIPager::new(5);

    //println!("Total: {unlinked_count} unmapped recordings");
    for record in &messy_recordings {
        let pager_continue = pager.execute(|| {
            println!(
                "({}) {} - {}",
                record.associated_listens.len(),
                record.messybrainz_data.recording,
                record.messybrainz_data.artist_credit
            );

            let latest_listen = record.get_latest_listen();

            println!(
                "    -> https://listenbrainz.org/user/{}/?min_ts={}&max_ts={}",
                username,
                latest_listen
                    .map(|listen| listen.listened_at - 1)
                    .unwrap_or(0),
                latest_listen
                    .map(|listen| listen.listened_at + 1)
                    .unwrap_or(0)
            );
            println!();
        });

        if !pager_continue {
            return;
        }
    }
}
