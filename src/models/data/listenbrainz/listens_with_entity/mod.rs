pub mod impl_trait;
pub mod map;
pub mod statistics;
pub mod traits;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use derive_getters::Getters;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

use super::listen::collection::mapped_primary_collection::PrimaryListenCollection;
use super::listen::collection::traits::CollectionOfListens;
use super::listen::primary_listen::PrimaryListen;

pub mod listens_with_recording;

#[derive(Debug, Clone, Getters)]
pub struct ListensWithEntity<E> {
    entity: Arc<E>,
    listens: PrimaryListenCollection,
}

impl<E> ListensWithEntity<E> {
    pub fn new_empty(entity: Arc<E>) -> Self {
        Self {
            entity,
            listens: Vec::new(),
        }
    }

    pub fn push(&mut self, listen: Arc<PrimaryListen>) {
        self.listens.push(listen);
    }

    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .find_oldest_listen()
            .map(|listen| *listen.get_listened_at())
    }

    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .find_latest_listen()
            .map(|listen| *listen.get_listened_at())
    }

    /// The number of listens of the entity
    pub fn listen_count(&self) -> usize {
        self.listens.len()
    }

    /// Return the amount of time this entity having been known about
    pub fn known_for(&self) -> Option<Duration> {
        self.first_listen_date()
            .map(|discovery| Utc::now() - discovery)
    }

    /// The average time between two listens of the entity
    pub fn average_duration_between_listens(&self) -> Duration {
        self.known_for()
            .and_then(|dur| dur.checked_div(self.listen_count() as i32))
            // If the recording haven't been listened to, then the average time is zero
            .unwrap_or_else(Duration::zero)
    }

    /// The date where the estimated next listen will be
    pub fn estimated_date_of_next_listen(&self) -> Option<DateTime<Utc>> {
        self.last_listen_date()
            .map(|listen_date| listen_date + self.average_duration_between_listens())
    }

    pub fn overdue_by(&self) -> Duration {
        self.estimated_date_of_next_listen()
            .map(|next_listen| Utc::now() - next_listen)
            .unwrap_or_else(Duration::zero)
    }

    /// Return `true` is the recording have been listened to
    pub fn is_listened(&self) -> bool {
        !self.listens.is_empty()
    }

    pub fn overdue_score(&self) -> Decimal {
        Decimal::from_i64(self.overdue_by().num_seconds()).unwrap()
            / Decimal::from_i64(self.average_duration_between_listens().num_seconds()).unwrap()
    }
}
