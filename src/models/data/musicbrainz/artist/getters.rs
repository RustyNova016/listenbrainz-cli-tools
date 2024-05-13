use itertools::Itertools;
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Browse;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertChildren;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::extensions::musicbrainz::BrowseQueryTExt;
use crate::utils::println_mus;

use super::Artist;

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
            recordings.push(Recording::get_cache().get_or_fetch(&id).await?);
        }
        Ok(recordings)
    }

    async fn fetch_all_recordings(&mut self) -> color_eyre::Result<()> {
        println_mus(format!("Getting {}'s recordings: {}", self.name, self.id));
        let recordings = RecordingMS::browse()
            .by_artist(&self.id)
            //.with_artists() // Broken! MusicBrainzRS consider that the includes are the same between fetch and browse!
            //.with_releases()
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
                .map(|recoding| recoding.id.into())
                .collect_vec(),
        );

        self.insert_into_cache().await?;
        Ok(())
    }
}
