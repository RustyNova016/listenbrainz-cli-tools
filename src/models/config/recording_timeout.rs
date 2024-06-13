use std::collections::HashMap;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;

use crate::core::entity_traits::config_file::ConfigFile;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordingTimeoutConfig(HashMap<RecordingMBID, DateTime<Utc>>);

impl RecordingTimeoutConfig {
    pub fn set_timeout(recording: RecordingMBID, duration: Duration) -> color_eyre::Result<()> {
        let mut conf = Self::load()?;
        conf.0.insert(recording, Utc::now() + duration);
        conf.save()?;

        Ok(())
    }

    pub fn get_timed_out_recordings() -> color_eyre::Result<Vec<RecordingMBID>> {
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
