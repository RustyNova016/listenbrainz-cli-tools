use chrono::{DateTime, Duration, Utc};
use derive_getters::Getters;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::{prelude::FromPrimitive, Decimal};


use crate::datastructures::listen_collection::{group_by::GroupByRecordingID, ListenCollection};

use super::impl_entity_with_listens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingWithListens {
    recording: Recording,
    listens: ListenCollection,
}

impl RecordingWithListens {
    pub fn new(recording: Recording, listens: ListenCollection) -> Self {
        Self { listens, recording }
    }

    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<Vec<Self>, crate::Error> {
        Ok(Self::from_group_by(listens.group_by_recording(conn).await?))
    }

    pub fn from_group_by(group_by: GroupByRecordingID) -> Vec<Self> {
        let mut res = Vec::new();

        for (_, (recording, listens)) in group_by {
            res.push(Self { listens, recording })
        }

        res
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

    /// Return the amount of time this recording having known about
    pub fn known_for(&self) -> Option<Duration> {
        self.first_listen_date()
            .map(|discovery| Utc::now() - discovery)
    }

    pub fn average_duration_between_listens(&self) -> Duration {
        // If the recording haven't been listened to, then the average time is zero
        if self.len() < 2 {
            return Duration::zero();
        }

        let duration_between_first_and_last = self
            .last_listen_date()
            .expect("There's at least two listens")
            - self
                .first_listen_date()
                .expect("There's at least two listens");

        duration_between_first_and_last
            .checked_div(self.len() as i32)
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

impl_entity_with_listens!(RecordingWithListens);