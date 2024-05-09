use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::MBID;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash,
)]
pub struct ReleaseGroupMBID(String);

impl MBID<ReleaseGroup> for ReleaseGroupMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<ReleaseGroup> {
        ReleaseGroup::get_cache().get_or_fetch(&self.0).await
    }
}
