use color_eyre::eyre::Context;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;
use musicbrainz_rs::Fetch;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::utils::println_mus;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct ReleaseGroupMBID(String);

impl IsMbid<ReleaseGroup> for ReleaseGroupMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<ReleaseGroup> {
        ReleaseGroup::get_cache().get_or_fetch(&self.0).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for work MBID: {}", &self));

        Ok(ReleaseGroupMS::fetch()
            .id(self)
            .with_releases()
            .execute()
            .await
            .context("Failed to fetch work from MusicBrainz")?
            .into_entity()
        )
    }

    fn into_mbid(self) -> MBID {
        MBID::ReleaseGroup(self)
    }
}
