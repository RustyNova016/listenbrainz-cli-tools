use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::core::entity_traits::cached::Cached;

use super::Recording;
use extend::ext;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RecordingMBID(String);

impl Deref for RecordingMBID {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl RecordingMBID {
    pub async fn get_recording(&self) -> color_eyre::Result<Recording> {
        Recording::get_cache().get_or_fetch(&self.0).await
    }
}
