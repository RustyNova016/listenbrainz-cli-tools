pub mod any_mbid;

use derive_more::{Display, From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;

pub mod converters;
pub mod extensions;
pub mod generic_mbid;
pub mod is_musicbrainz_id;
pub mod primary;

#[derive(Debug, Clone, PartialEq, Eq, From, Serialize, Deserialize, Display, IsVariant, Unwrap)]
pub enum MBID {
    Artist(ArtistMBID),
    Release(ReleaseMBID),
    Work(WorkMBID),
    ReleaseGroup(ReleaseGroupMBID),
    Recording(RecordingMBID),
}

impl IsMbid<MusicBrainzEntity> for MBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<MusicBrainzEntity> {
        Ok(match self {
            Self::Artist(val) => val.get_or_fetch_entity().await?.into(),
            Self::Release(val) => val.get_or_fetch_entity().await?.into(),
            Self::Work(val) => val.get_or_fetch_entity().await?.into(),
            Self::ReleaseGroup(val) => val.get_or_fetch_entity().await?.into(),
            Self::Recording(val) => val.get_or_fetch_entity().await?.into(),
        })
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        match self {
            Self::Artist(val) => val.fetch().await,
            Self::Release(val) => val.fetch().await,
            Self::Work(val) => val.fetch().await,
            Self::ReleaseGroup(val) => val.fetch().await,
            Self::Recording(val) => val.fetch().await,
        }
    }

    fn into_mbid(self) -> MBID {
        self
    }
}
