use crate::models::cli::common::SortListensBy;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::utils::println_cli;
use core::fmt;
use inquire::Select;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub async fn interactive_mapper(username: &str, token: String, sort: Option<SortListensBy>) {
    println_cli(format!("Fetching unmapped for user {username}"));
    let mut unmappeds: ListenCollection = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_unmapped_listens()
        .into_iter()
        .unique_by(|listen| listen.get_messybrain_data().msid.clone())
        .collect();

    unmappeds.sort_by_criteria(sort.unwrap_or_default());

    let mut i = 0;
    loop {
        let Some(listen) = unmappeds.get(i) else {
            break;
        };

        match process_listen(username, listen.clone(), &token)
            .await
            .expect("Couldn't map listen")
        {
            ReturnOption::Exit => break,
            ReturnOption::Next => i += 1,
            ReturnOption::Previous => i -= 1,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct SearchResultItem {
    pub(self) recording_name: String,
    pub(self) recording_mbid: String,
    pub(self) artist_credit_name: String,
}

impl Display for SearchResultItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.recording_name, self.artist_credit_name)
    }
}

async fn process_listen(
    username: &str,
    listen: Arc<Listen>,
    token: &str,
) -> color_eyre::Result<ReturnOption> {
    let client = reqwest::Client::new();
    let mut req_body = HashMap::new();
    req_body.insert(
        "query",
        format!(
            "{} - {}",
            listen.get_messybrain_data().track_name,
            listen.get_messybrain_data().artist_name
        ),
    );

    let res: Vec<SearchResultItem> = client
        .post("https://labs.api.listenbrainz.org/recording-search/json")
        .json(&vec![req_body])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let mut options = res;

    options.push(SearchResultItem {
        recording_name: "⏮️  Previous".to_owned(),
        recording_mbid: "-".to_owned(),
        artist_credit_name: "Exit the program".to_owned(),
    });

    options.push(SearchResultItem {
        recording_name: "⏭️  Skip".to_owned(),
        recording_mbid: "0".to_owned(),
        artist_credit_name: "Skip the current listen".to_owned(),
    });
    options.push(SearchResultItem {
        recording_name: "❌ Exit".to_owned(),
        recording_mbid: "-1".to_owned(),
        artist_credit_name: "Exit the program".to_owned(),
    });

    println!();
    println!();
    println!("Current unmapped recording:");
    println!("  - Title: {}", listen.get_messybrain_data().track_name);
    println!("  - Artist: {}", listen.get_messybrain_data().artist_name);
    println!(
        "    -> https://listenbrainz.org/user/{}/?max_ts={}",
        username,
        listen.listened_at.timestamp() + 1
    );
    println!();
    println!();
    let ans = Select::new("Select a recording", options)
        .prompt()
        .expect("There was an error");

    if ans.recording_mbid == "0" {
        return Ok(ReturnOption::Next);
    } else if ans.recording_mbid == "-1" {
        return Ok(ReturnOption::Exit);
    } else if ans.recording_mbid == "-2" {
        return Ok(ReturnOption::Previous);
    } else {
        // Map the recording
        listen.submit_mapping(&ans.recording_mbid, token).await?;
    }

    Ok(ReturnOption::Next)
}

enum ReturnOption {
    Next,
    Previous,
    Exit,
}
