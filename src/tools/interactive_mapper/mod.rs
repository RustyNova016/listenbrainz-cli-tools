use std::collections::HashMap;

use inquire::{InquireError, Select};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{models::cache::global_cache::GlobalCache, utils::println_cli};

pub async fn interactive_mapper(username: &str) {
    println_cli(format!("Fetching unmapped for user {}", username));
    let unmappeds = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens")
        .expect("Couldn't fetch the new listens")
        .get_unmapped_listens();

    for listen in unmappeds {
        let client = reqwest::Client::new();
        let mut req_body = HashMap::new();
        req_body.insert("query", format!("{} - {}", listen.get_messybrain_data().track_name, listen.get_messybrain_data().artist_name));

        let res: Vec<SearchResultItem> = client
            .post("https://labs.api.listenbrainz.org/recording-search/json")
            .json(&vec![req_body])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let mut options = res.into_iter().map(|result| format!("{} - {}", result.recording_name, result.artist_credit_name)).collect_vec();
        options.push("Skip".to_owned());
        options.push("Exit".to_owned());

        let _ans: Result<String, InquireError> = Select::new("Select a recording", options).prompt();
    
        
    }
}

#[derive(Deserialize, Serialize)]
struct SearchResultItem {
    recording_name: String,
    recording_mbid: String,
    artist_credit_name: String
}