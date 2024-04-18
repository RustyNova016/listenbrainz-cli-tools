use std::sync::Arc;

use itertools::Itertools;
use musicbrainz_rs::entity::label::LabelInfo;
use strsim::sorensen_dice;

use crate::{
    models::{cache::global_cache::GlobalCache, data::listenbrainz::listen::Listen},
    utils::cli_paging::CLIPager,
};

pub async fn wrong_link(username: String) {
    // Get the listens
    let mapped_listens = GlobalCache::new()
        .get_user_listens_with_refresh(&username)
        .expect("Couldn't fetch the new listens")
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let mut mapped_with_score: Vec<(u64, Arc<Listen>)> = mapped_listens
        .into_iter()
        .map(|listen| {
            let origin_string = format!(
                "{} - {}",
                listen.get_messybrain_data().track_name,
                listen.get_messybrain_data().artist_name
            );

            let mapping_data = listen
                .get_mapping_data()
                .as_ref()
                .expect("All the listens are expected to be mapped, but this on isn't!");
            let mapped_string = format!(
                "{} - {}",
                mapping_data.recording_name.clone(),
                mapping_data.artist_credit.clone().unwrap_or("".to_string())
            );

            let score = sorensen_dice(&origin_string.to_lowercase(), &mapped_string.to_lowercase());

            ((score * 1000_f64) as u64, listen)
        })
        .collect_vec();

    mapped_with_score.sort_by_key(|a| a.0);

    let mut pager = CLIPager::new(5);
    for (score, listen) in mapped_with_score {
        let pager_continue = pager.execute(|| {
            println!(
                "[{}] \r\n {} - {} -> Mapping: {} - {}",
                score,
                listen.get_messybrain_data().track_name,
                listen.get_messybrain_data().artist_name,
                listen
                    .get_mapping_data()
                    .as_ref()
                    .unwrap()
                    .get_recording_name(),
                listen
                    .get_mapping_data()
                    .as_ref()
                    .unwrap()
                    .artist_credit
                    .as_ref()
                    .unwrap_or(&"".to_string())
            )
        });

        if !pager_continue {
            return;
        };
    }
}
