use color_eyre::eyre::{eyre, Context, OptionExt};
use itertools::Itertools;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::Release;

impl Release {
    pub fn get_recording_ids(&self) -> Option<Vec<RecordingMBID>> {
        self.media.as_ref().map(|medias| {
            medias
                .iter()
                .flat_map(|media| media.get_recording_mbids().unwrap_or_default())
                .collect_vec()
        })
    }

    pub async fn get_or_fetch_recording_ids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        Ok(match self.get_recording_ids() {
            Some(recordings) => recordings,
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .get_recording_ids()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Media field is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}
