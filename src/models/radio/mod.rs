use std::cmp::Reverse;

use crate::utils::println_cli_info;

use super::config::recording_timeout::RecordingTimeoutConfig;
use super::config::Config;
use super::data::musicbrainz::recording::mbid::RecordingMBID;
use super::data::musicbrainz::recording::Recording;
use chrono::Duration;
use derive_builder::Builder;
use futures::Stream;
use futures::StreamExt;
use itertools::Itertools;
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
    pub fn check_min_lenght(&self, playlist: &[Recording]) -> bool {
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

    pub fn sort_scores(recordings: Vec<(Decimal, RecordingMBID)>) -> Vec<RecordingMBID> {
        let conf = Config::load_or_panic();

        #[allow(clippy::clone_on_copy)] // Borrow checker doesn't like the implicit copy of score
        recordings
            .into_iter()
            .map(|(score, recording)| (score * conf.bumps.get_multiplier(&recording), recording))
            .sorted_by_key(|(score, _)| Reverse(score.clone()))
            .map(|(_, r)| r)
            .collect()
    }

    pub async fn finalize_radio_playlist<I, E>(&self, mut generator: I) -> Result<Vec<Recording>, E>
    where
        I: Stream<Item = Result<Recording, E>> + Unpin,
        E: Sync + Send,
    {
        let mut results = Vec::new();
        let timed_out_recordings = RecordingTimeoutConfig::get_timed_out_recordings()
            .expect("Couldn't fetch the timeout config");
        #[cfg(debug_assertions)]
        println_cli_info(format!("Found {} timeouts", timed_out_recordings.len()));

        while let Some(recording) = generator.next().await.transpose()? {
            if timed_out_recordings.contains(recording.id()) {
                println_cli_info(format!(
                    "Ignoring {} ({}). Recording in timeout",
                    recording.title(),
                    recording.id()
                ));
                continue;
            }

            results.push(recording);

            if self.check_min_lenght(&results) {
                return Ok(results);
            }
        }

        Ok(results)
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
