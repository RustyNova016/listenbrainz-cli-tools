use chrono::Duration;
use clap::ValueEnum;
use derive_getters::Getters;
use derive_more::IsVariant;
use derive_new::new;
use rust_decimal::prelude::Decimal;

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
        Decimal::from(self.listen_count) * multiplier
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
