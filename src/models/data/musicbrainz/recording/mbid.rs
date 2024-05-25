use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::utils::println_mus;
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::external::RecordingExt;

use super::Recording;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct RecordingMBID(String);

impl IsMbid<Recording> for RecordingMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Recording> {
        Recording::get_cache().get_or_fetch(&self.0).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for recording MBID: {}", &self));

        color_eyre::eyre::Ok(RecordingMS::fetch()
            .id(self)
            .with_artists()
            .with_releases()
            .with_work_relations()
            .with_aliases()
            .with_work_level_relations()
            .execute()
            .await
            .context("Failed to fetch recording from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::Recording(self)
    }
}