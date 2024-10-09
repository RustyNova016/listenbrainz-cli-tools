use std::collections::HashMap;

use musicbrainz_db_lite::models::musicbrainz::{label::Label, recording::Recording, release::Release};
use sqlx::SqliteConnection;

use crate::utils::cli::global_progress_bar::PG_FETCHING;

use super::ListenCollection;

pub type GroupByLabelID = HashMap<i64, (Label, ListenCollection)>;
pub type GroupByReleaseID = HashMap<i64, (Release, ListenCollection)>;
pub type GroupByRecordingID = HashMap<i64, (Recording, ListenCollection)>;

impl ListenCollection {
    pub async fn group_by_recording(
        self,
        conn: &mut SqliteConnection,
    ) -> Result<GroupByRecordingID, musicbrainz_db_lite::Error> {
        let mut res = HashMap::new();
        let progress_bar = PG_FETCHING.get_submitter(self.data.len() as u64);

        for listen in self.data {
            let Some(recording) = listen.get_recording_or_fetch(conn).await? else {continue};
            let bucket = res.entry(recording.id);

            bucket
                .or_insert_with(|| (recording, ListenCollection::new(Vec::new())))
                .1
                .data
                .push(listen);
            progress_bar.inc(1);
        }

        Ok(res)
    }

    pub async fn group_by_release(
        self,
        conn: &mut SqliteConnection,
    ) -> Result<GroupByReleaseID, musicbrainz_db_lite::Error> {
        let mut res = HashMap::new();
        let progress_bar = PG_FETCHING.get_submitter(self.data.len() as u64);

        for listen in self.data {
            let Some(recording) = listen.get_recording_or_fetch(conn).await? else {continue};
            let releases = recording.get_releases_or_fetch(conn).await?;

            for release in releases {
                let bucket = res.entry(release.id);

                bucket
                    .or_insert_with(|| (release, ListenCollection::new(Vec::new())))
                    .1
                    .data
                    .push(listen.clone());
                
            }

            progress_bar.inc(1);
        }

        Ok(res)
    }
}

