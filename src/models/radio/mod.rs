use std::cmp::Reverse;

use crate::utils::println_cli_info;

use super::config::recording_timeout::RecordingTimeoutConfig;

use super::config::Config;
use super::data::musicbrainz::recording::mbid::RecordingMBID;

use super::data::musicbrainz::recording::Recording as RecordingOld;

use chrono::Duration;
use derive_builder::Builder;
use futures::stream;
use futures::Stream;
use futures::StreamExt;

use itertools::Itertools;

use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use once_cell::sync::Lazy;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct RadioConfig {
    #[builder(default)]
    min_count: u64,

    #[builder(default)]
    min_duration: Duration,

    #[builder(default)]
    min_mode: MinimumMode,
}

impl RadioConfig {
    #[deprecated]
    pub fn check_min_lenght(&self, playlist: &[RecordingOld]) -> bool {
        let has_min_count = playlist.len() as u64 >= self.min_count;
        let has_min_duration = Lazy::new(|| {
            playlist
                .iter()
                .map(|recording| recording.get_duration().unwrap_or_default())
                .sum::<Duration>()
                >= self.min_duration
        });

        match self.min_mode {
            MinimumMode::Or => has_min_count || *has_min_duration,
            MinimumMode::And => has_min_count && *has_min_duration,
        }
    }
}

impl Default for RadioConfig {
    fn default() -> Self {
        Self {
            min_count: 50,
            min_duration: Default::default(),
            min_mode: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum MinimumMode {
    #[default]
    And,
    Or,
}
