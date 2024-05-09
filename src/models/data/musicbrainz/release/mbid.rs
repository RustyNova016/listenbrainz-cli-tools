use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::MBID;
use crate::models::data::musicbrainz::release::Release;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize)]
pub struct ReleaseMBID(String);

impl MBID<Release> for ReleaseMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Release> {
        Release::get_cache().get_or_fetch(&self.0).await
    }
}
