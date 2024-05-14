use super::Work;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::MBID;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct WorkMBID(String);

impl MBID<Work> for WorkMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Work> {
        Work::get_cache().get_or_fetch(&self.0).await
    }
}
