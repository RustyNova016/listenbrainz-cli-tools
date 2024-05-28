use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::MBID;

use super::Recording;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct RecordingMBID(String);

impl MBID<Recording> for RecordingMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Recording> {
        Recording::get_cache().get_or_fetch(&self.0).await
    }
}
