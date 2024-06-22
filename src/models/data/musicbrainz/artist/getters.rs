use std::sync::Arc;

use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Browse;

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::external_musicbrainz_entity::FlattenedMBEntityExt;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::extensions::musicbrainz_ext::BrowseQueryTExt;
use crate::utils::println_mus;

use super::Artist;

impl Artist {
    pub async fn get_all_recordings(&mut self) -> color_eyre::Result<Vec<Arc<Recording>>> {
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
            recordings.push(id.get_or_fetch_entity().await?);
        }
        Ok(recordings)
    }

    async fn fetch_all_recordings(&mut self) -> color_eyre::Result<()> {
        println_mus(format!("Getting {}'s recordings: {}", self.name, self.id));
        let recordings = RecordingMS::browse()
            .by_artist(&self.id)
            //.with_artists() //TODO: Broken! MusicBrainzRS consider that the includes are the same between fetch and browse!
            //.with_releases()
            .execute_all(100)
            .await?;

        let mut recording_ids = Vec::new();
        for recording in recordings.entities.into_iter() {
            let flattened = recording.flattened();
            recording_ids.push(flattened.0.get_mbid().unwrap_recording());
            flattened.insert_into_cache().await?;
        }

        self.recordings = Some(recording_ids);

        self.save().await?;
        Ok(())
    }
}
