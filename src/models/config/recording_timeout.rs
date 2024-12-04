use std::collections::HashMap;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;

use crate::models::config::ConfigFile;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordingTimeoutConfig(HashMap<String, DateTime<Utc>>);

impl RecordingTimeoutConfig {
    pub fn set_timeout(recording: &str, duration: Duration) -> color_eyre::Result<()> {
        let mut conf = Self::load()?;
        conf.0.insert(recording.to_string(), Utc::now() + duration);
        conf.save()?;

        Ok(())
    }

    pub fn get_timed_out_recordings() -> color_eyre::Result<Vec<String>> {
        let conf = Self::load()?;

        Ok(conf
            .0
            .into_iter()
            .filter_map(|(id, deadline)| {
                if Utc::now() < deadline {
                    return Some(id);
                }
                None
            })
            .collect_vec())
    }
}

impl ConfigFile for RecordingTimeoutConfig {
    fn file_name() -> &'static str {
        "timeouts.json"
    }
}
