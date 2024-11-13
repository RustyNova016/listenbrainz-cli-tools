use chrono::Duration;
use futures::StreamExt;
use macon::Builder;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;

#[derive(Debug, Builder)]
pub struct RadioCollector {
    duration: Option<Duration>,
    count: Option<u64>,
}

impl RadioCollector {
    pub async fn collect(
        &self,
        mut recordings: impl StreamExt<Item = RecordingWithListens> + Unpin,
    ) -> Vec<Recording> {
        let mut results = Vec::new();
        while let Some(recording) = recordings.next().await {
            results.push(recording.recording().clone());

            if self.check_minimum_lenght(&results) {
                return results;
            }
        }

        results
    }

    /// Return true if the lenght of the playlist satisfy the requested minimum time
    pub fn check_minimum_lenght(&self, playlist: &[Recording]) -> bool {
        let has_min_count = match self.count {
            Some(count) => playlist.len() as u64 >= count,
            None => self.duration.is_some() || playlist.len() as u64 >= 50,
        };

        if !has_min_count {
            return false;
        }

        match self.duration {
            Some(duration) => {
                playlist
                    .iter()
                    .map(|recording| recording.length_as_duration().unwrap_or_default())
                    .sum::<Duration>()
                    >= duration
            }
            None => true,
        }
    }
}
