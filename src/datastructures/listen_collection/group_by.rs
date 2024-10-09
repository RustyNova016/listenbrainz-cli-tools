use std::collections::{hash_map::IntoIter, HashMap};

use musicbrainz_db_lite::models::{
    listenbrainz::listen::Listen,
    musicbrainz::{label::Label, recording::Recording, release::Release},
};
use sqlx::SqliteConnection;

use crate::utils::cli::global_progress_bar::PG_FETCHING;

use super::ListenCollection;

pub type GroupByLabelID = HashMap<i64, (Label, ListenCollection)>;
pub type GroupByRecordingID = GroupBy<Recording, ListenCollection>;
pub type GroupByReleaseID = GroupBy<Release, GroupByRecordingID>;

impl ListenCollection {
    pub async fn group_by_recording(
        self,
        conn: &mut SqliteConnection,
    ) -> Result<GroupByRecordingID, musicbrainz_db_lite::Error> {
        let mut group: GroupByRecordingID = GroupBy::default();
        let progress_bar = PG_FETCHING.get_submitter(self.data.len() as u64);

        for listen in self.data {
            let Some(recording) = listen.get_recording_or_fetch(conn).await? else {
                continue;
            };
            group
                .get_mut_col_or_create(recording.id, recording)
                .push(listen);

            progress_bar.inc(1);
        }

        Ok(group)
    }

    pub async fn group_by_release(
        self,
        conn: &mut SqliteConnection,
    ) -> Result<GroupByReleaseID, musicbrainz_db_lite::Error> {
        let mut group: GroupByReleaseID = GroupBy::default();
        let progress_bar = PG_FETCHING.get_submitter(self.data.len() as u64);

        for listen in self.data {
            let Some(recording) = listen.get_recording_or_fetch(conn).await? else {
                continue;
            };

            for release in recording.get_releases_or_fetch(conn).await? {
                group
                    .get_mut_col_or_create(release.id, release)
                    .get_mut_col_or_create(recording.id, recording.clone())
                    .push(listen.clone())
            }

            progress_bar.inc(1);
        }

        Ok(group)
    }
}

#[derive(Debug)]
pub struct GroupBy<T, Col> {
    data: HashMap<i64, (T, Col)>,
}

impl<T, Col: Default> GroupBy<T, Col> {
    pub fn get_mut_or_create(&mut self, k: i64, ent: T) -> &mut (T, Col) {
        self.data
            .entry(k)
            .or_insert_with(|| (ent, Default::default()))
    }

    pub fn get_mut_col_or_create(&mut self, k: i64, ent: T) -> &mut Col {
        &mut self.get_mut_or_create(k, ent).1
    }
}

impl GroupBy<Recording, ListenCollection> {
    pub fn insert(&mut self, k: i64, entity: Recording, listen: Listen) {
        let bucket = self.data.entry(k);

        bucket
            .or_insert_with(|| (entity, ListenCollection::default()))
            .1
            .push(listen.clone());
    }
}

impl<T, Col> Default for GroupBy<T, Col> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<T, Col> IntoIterator for GroupBy<T, Col> {
    type Item = (i64, (T, Col));
    type IntoIter = IntoIter<i64, (T, Col)>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
