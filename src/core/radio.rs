use std::sync::Arc;

use chrono::{Duration, Utc};
use derive_builder::Builder;

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::cli::config::SelfEditContext;
use crate::models::config::CONFIG;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

pub async fn apply_radio_insert_config(recording_ids: Vec<RecordingMBID>) -> Vec<RecordingMBID> {
    let mut results = Vec::new();

    for recording_id in recording_ids {
        if results.len() >= 50 {
            break;
        } //TODO: Add radio limit to config;

        let Some(recording_id) = CONFIG
            .read()
            .await
            .self_edits()
            .apply_action_for_context(recording_id.into_mbid(), &SelfEditContext::RadioInsert)
        else {
            continue;
        };

        results.push(recording_id.unwrap_recording());
    }

    results
}

pub async fn apply_radio_seeeding_config(seeds: Vec<MBID>) -> Vec<MBID> {
    let mut results = Vec::new();

    for seed in seeds {
        let Some(seed) = CONFIG
            .read()
            .await
            .self_edits()
            .apply_action_for_context(seed, &SelfEditContext::RadioSeeding)
        else {
            continue;
        };

        results.push(seed);
    }

    results
}

#[derive(Debug, Clone, Builder)]
pub struct ListenSeeds {
    username: String,
    cooldown: u64,
    mapped: bool,
    unmapped: bool,
}

impl ListenSeeds {
    pub async fn get_seeds(&self) -> ListenCollection {
        let listens = UserListens::get_user_with_refresh(&self.username)
            .await
            .expect("Couldn't fetch the new listens")
            .get_listens()
            .clone();

        let mut results = Vec::new();

        for listen in listens {
            if listen.is_mapped() && !self.mapped {
                continue;
            }

            if !listen.is_mapped() && !self.unmapped {
                continue;
            }

            let Some(new_listen) = self.remap_listen(listen).await else {
                continue;
            };
        }

        ListenCollection::from_iter(results)
    }

    async fn remap_listen(&self, listen: Arc<Listen>) -> Option<Arc<Listen>> {
        let Some(recording_id) = listen.get_primary_recording_id().await.unwrap() else {
            return Some(listen);
        };

        let new_id = CONFIG
            .read()
            .await
            .self_edits()
            .apply_action_for_context(recording_id.into_mbid(), &SelfEditContext::RadioSeeding)?;

        let mut new_listen = listen.as_ref().clone();
        new_listen.set_recording_mapping(new_id.unwrap_recording()); //TODO: Prevent unwrap
        Some(Arc::new(new_listen))
    }

    async fn get_recordings_in_cooldown(self, listens: &ListenCollection) -> Vec<RecordingMBID> {
        let deadline = Utc::now() - Duration::hours(self.cooldown as i64);

        let mut blacklist = Vec::new();

        for listen in listens.iter() {
            if listen.get_listened_at() > &deadline {
                if let Some(id) = listen.get_primary_recording_id().await.unwrap() {
                    // TODO: Error message
                    blacklist.push(id)
                }
            }
        }

        blacklist
    }
}
