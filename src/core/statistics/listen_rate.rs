use std::ops::Div;
use chrono::{DateTime, Duration, Utc};
use clap::ValueEnum;
use derive_getters::Getters;
use derive_more::IsVariant;
use derive_new::new;
use rust_decimal::prelude::Decimal;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

#[derive(Debug, Clone, Getters, PartialEq, Eq, new)]
pub struct ListenRate {
    recording: RecordingMBID,
    listen_count: u64,
    duration: Duration,
}

impl ListenRate {
    pub fn get_listen_rate(&self, rate: ListenRateRange) -> Decimal {
        let multiplier = Decimal::from(rate.get_duration().num_seconds())
            / Decimal::from(self.duration.num_seconds());
        Decimal::from(*self.listen_count()) * multiplier
    }
    
    pub fn get_average_time_between_listens(&self) -> Duration {
        self.duration.div(*self.listen_count() as i32)
    }
    
    pub fn get_estimated_date_of_next_listen(&self, listen_collection: &ListenCollection) -> DateTime<Utc> {
        let latest_listen_date = listen_collection.get_latest_listen().map(|listen| listen.listened_at).unwrap_or(Utc::now());
        latest_listen_date + self.get_average_time_between_listens()
    }
}

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum ListenRateRange {
    Year,
    Month,
}

impl ListenRateRange {
    pub fn get_duration(&self) -> Duration {
        match self {
            Self::Year => Duration::days(365),
            Self::Month => Duration::days(30),
        }
    }
}
