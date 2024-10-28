use std::collections::HashMap;

use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::RowId;

use crate::database::listenbrainz::prefetching::prefetch_releases;
use crate::datastructures::listen_collection::ListenCollection;

use super::recording_with_listens::RecordingWithListens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct ReleaseWithListens {
    release: Release,
    listens: Vec<RecordingWithListens>,
}

impl ReleaseWithListens {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<HashMap<i64, Self>, crate::Error> {
        // Convert Recordings
        let recordings = RecordingWithListens::from_listencollection(conn, listens).await?;

        // Prefetch Releases
        let recording_refs = recordings.values().map(|r| r.recording()).collect_vec();
        prefetch_releases(conn, &recording_refs).await?;

        // Load Releases
        let results = Recording::get_releases_as_batch(conn, &recording_refs).await?;

        // Convert releases
        let mut out = HashMap::new();

        for (_, (recording, releases)) in results {
            for release in releases {
                out.entry(release.get_row_id())
                    .or_insert_with(|| Self {
                        release,
                        listens: Vec::new(),
                    })
                    .push(recordings.get(&recording.id).expect("The release has been fetched from the recording, so it should be there").clone());
            }
        }

        Ok(out)
    }

    pub fn push(&mut self, value: RecordingWithListens) {
        self.listens.push(value);
    }

    /// Return the listen count
    pub fn len(&self) -> usize {
        self.listens.iter().map(|r| r.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

//impl_entity_with_listens!(ReleaseWithListens);
