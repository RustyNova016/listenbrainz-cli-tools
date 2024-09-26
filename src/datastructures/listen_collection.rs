use std::collections::HashMap;

use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use sqlx::SqliteConnection;

use crate::core::display::progress_bar::ProgressBarCli;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListenCollection {
    pub data: Vec<Listen>,
}

pub type GroupByRecordingID = HashMap<i64, ListenCollection>;

impl ListenCollection {
    /// Returns the latest listen in the collection.
    pub fn get_latest_listen(&self) -> Option<&Listen> {
        self.data.iter().max_by_key(|listen| listen.listened_at)
    }

    /// Returns the oldest listen in the collection.
    pub fn get_oldest_listen(&self) -> Option<&Listen> {
        self.data.iter().min_by_key(|listen| listen.listened_at)
    }

    pub async fn group_by_recording(
        self,
        conn: &mut SqliteConnection,
    ) -> Result<GroupByRecordingID, musicbrainz_db_lite::Error> {
        let mut res: HashMap<i64, ListenCollection> = HashMap::new();

        let progress_bar =
            ProgressBarCli::new(self.data.len() as u64, Some("Fetching listened recordings")); // TODO: Better progress handling

        for listen in self.data {
            let recording = listen.get_recording_or_fetch(conn).await?;
            let bucket = res.entry(recording.map(|r| r.id).unwrap_or(-1));
            bucket.or_default().data.push(listen);
            progress_bar.inc(1);
        }

        Ok(res)
    }
}

impl From<Vec<Listen>> for ListenCollection {
    fn from(value: Vec<Listen>) -> Self {
        Self { data: value }
    }
}
