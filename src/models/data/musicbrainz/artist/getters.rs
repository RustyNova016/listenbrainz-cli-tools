use super::Artist;
use crate::core::entity_traits::insertable::IsAutoInsertable;
use crate::{
    core::entity_traits::{
        cached::Cached, has_id::HasID, insertable::Insertable, insertable_children::InsertChildren,
    },
    models::data::musicbrainz::recording::Recording,
    utils::extensions::musicbrainz::BrowseQueryTExt,
};
use itertools::Itertools;
use musicbrainz_rs::{entity::recording::Recording as RecordingMS, Browse};
use crate::utils::println_mus;

impl Artist {
    pub async fn get_all_recordings(&mut self) -> color_eyre::Result<Vec<Recording>> {
        let recording_ids = match &self.recordings {
            Some(recordings) => recordings.clone(),
            None => {
                self.fetch_all_recordings().await?;
                self.recordings
                    .clone()
                    .expect("Couldn't retrive the recordings after insertion")
            }
        };

        let mut recordings = Vec::new();
        for id in recording_ids {
            recordings.push(Recording::get_cache().get_or_fetch(&id).await?)
        }
        Ok(recordings)
    }

    async fn fetch_all_recordings(&mut self) -> color_eyre::Result<()> {
        println_mus(format!("Browsing: {}", self.id));
        let recordings = RecordingMS::browse()
            .by_artist(&self.id)
            .execute_all(100)
            .await?;

        for recording in recordings.entities.clone() {
            InsertChildren::from(recording.clone())
                .insert_into_cache_as(recording.get_id())
                .await?;
        }

        self.recordings = Some(
            recordings
                .entities
                .into_iter()
                .map(|recoding| recoding.id)
                .collect_vec(),
        );

        self.insert_into_cache().await?;
        Ok(())
    }
}
