use std::sync::Arc;

use derive_more::{Display, From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;

use super::entity::IsMusicbrainzEntity;

pub mod converters;
pub mod extensions;
pub mod generic_mbid;
pub mod mbid_kind;

#[derive(
    Debug, Clone, PartialEq, Eq, From, Serialize, Deserialize, Display, IsVariant, Unwrap, Hash,
)]
pub enum MBIDEnum {
    Artist(ArtistMBID),
    Release(ReleaseMBID),
    Work(WorkMBID),
    ReleaseGroup(ReleaseGroupMBID),
    Recording(RecordingMBID),
}

impl IsMbid<AnyMusicBrainzEntity> for MBIDEnum {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Arc<AnyMusicBrainzEntity>> {
        Ok(Arc::new(match self {
            Self::Artist(val) => AnyMusicBrainzEntity::from(val.get_or_fetch_entity().await?),
            Self::Release(val) => AnyMusicBrainzEntity::from(val.get_or_fetch_entity().await?),
            Self::Work(val) => AnyMusicBrainzEntity::from(val.get_or_fetch_entity().await?),
            Self::ReleaseGroup(val) => AnyMusicBrainzEntity::from(val.get_or_fetch_entity().await?),
            Self::Recording(val) => AnyMusicBrainzEntity::from(val.get_or_fetch_entity().await?),
        }))
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

    fn into_mbid(self) -> MBIDEnum {
        self
    }
}

pub struct MBID<K: IsMusicbrainzID<T>, T: IsMusicbrainzEntity> (K);

pub trait IsMusicbrainzID<T> {}