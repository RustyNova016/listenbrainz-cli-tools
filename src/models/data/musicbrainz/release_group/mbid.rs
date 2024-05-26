use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::utils::println_mus;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct ReleaseGroupMBID(String);

impl IsMbid<ReleaseGroup> for ReleaseGroupMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<ReleaseGroup> {
        ReleaseGroup::get_cached_or_fetch(self).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for work MBID: {}", &self));

        Ok(ReleaseGroupMS::fetch()
            .id(self)
            .with_releases()
            .execute()
            .await
            .context("Failed to fetch work from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::ReleaseGroup(self)
    }
}
