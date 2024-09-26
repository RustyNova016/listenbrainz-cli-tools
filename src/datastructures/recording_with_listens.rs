use chrono::{DateTime, Duration, Utc};
use derive_getters::Getters;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use sqlx::SqliteConnection;

use super::listen_collection::{GroupByRecordingID, ListenCollection};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingWithListens {
    recording: Recording,
    listens: ListenCollection
}

impl RecordingWithListens {
    pub fn new(recording: Recording, listens: ListenCollection) -> Self {
        Self {
            listens,
            recording
        }
    }

    pub async fn from_group_by(conn: &mut SqliteConnection, group_by: GroupByRecordingID) -> Result<Vec<Self>, crate::Error> {
        let mut res = Vec::new();

        for (id, listens) in group_by {
            // Ignore unmapped
            if id == -1 {continue}
            
            res.push(Self {
                listens,
                recording: Recording::find_by_id_column(conn, id).await?.ok_or(crate::Error::MissingRowInDB(id))?
            })
        }

        Ok(res)
    }

    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .get_oldest_listen()
            .map(|listen| listen.listened_at_as_datetime())
    }

    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .get_latest_listen()
            .map(|listen| listen.listened_at_as_datetime())
    }

    pub fn listen_count(&self) -> usize {
        self.listens.data.len()
    }

    /// Return the amount of time this recording having known about
    pub fn known_for(&self) -> Option<Duration> {
        self.first_listen_date()
            .map(|discovery| Utc::now() - discovery)
    }

    pub fn average_duration_between_listens(&self) -> Duration {
        self.known_for()
            .and_then(|dur| dur.checked_div(self.listen_count() as i32))
            // If the recording haven't been listened to, then the average time is zero
            .unwrap_or_else(Duration::zero)
    }

    pub fn estimated_date_of_next_listen(&self) -> Option<DateTime<Utc>> {
        self.last_listen_date()
            .map(|listen_date| listen_date + self.average_duration_between_listens())
    }

    pub fn overdue_by(&self) -> Duration {
        self.estimated_date_of_next_listen()
            .map(|next_listen| Utc::now() - next_listen)
            .unwrap_or_else(Duration::zero)
    }

    pub fn overdue_factor(&self) -> Decimal {
        Decimal::from_i64(self.overdue_by().num_seconds()).unwrap()
        / Decimal::from_i64(self.average_duration_between_listens().num_seconds()).unwrap()
    }

    pub fn is_listened(&self) -> bool {
        !self.listens.data.is_empty()
    }

/*     pub async fn underated_score_single(&self) -> color_eyre::Result<Decimal> {
        Ok(self
            .listens()
            .get_underrated_recordings()
            .await?
            .first()
            .expect("Recording should have a score")
            .0)
    } */
}