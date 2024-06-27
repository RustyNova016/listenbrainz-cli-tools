use std::mem::discriminant;
use std::ops::Deref;
use std::sync::Arc;

use derive_more::{From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mbid::HasMBID;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::any_mbid::AnyMBIDType;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::utils::println_cli_warn;

use super::entity_kind::MusicbrainzEntityKind;
use super::is_musicbrainz_entity::IsMusicbrainzEntity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IsVariant, Unwrap, From)]
pub enum AnyMusicBrainzEntity {
    Artist(Arc<Artist>),
    ReleaseGroup(Arc<ReleaseGroup>),
    Release(Arc<Release>),
    Recording(Arc<Recording>),
    Work(Arc<Work>),
}

impl AnyMusicBrainzEntity {
    pub async fn save_to_cache(&self) -> color_eyre::Result<()> {
        match self {
            Self::ReleaseGroup(val) => MUSICBRAINZ_DATABASE.release_groups().update(val).await?,
            Self::Release(val) => MUSICBRAINZ_DATABASE.releases().update(val).await?,
            Self::Recording(val) => MUSICBRAINZ_DATABASE.recordings().update(val).await?,
            Self::Work(val) => MUSICBRAINZ_DATABASE.works().update(val).await?,
            Self::Artist(val) => MUSICBRAINZ_DATABASE.artists().update(val).await?,
        }

        Ok(())
    }

    pub fn update(self, newer: Self) -> Self {
        // Check if both are the same variant
        if discriminant(&self) != discriminant(&newer) {
            // No big deal. But worth mentioning
            // TODO: Debug only warning
            println_cli_warn("Tried to update entity type with mismatched entity type".to_string());
            return self;
        }

        match self {
            Self::Artist(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_artist().as_ref().clone())
                .into_arc_and_any(),
            Self::Recording(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_recording().as_ref().clone())
                .into_arc_and_any(),
            Self::Release(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_release().as_ref().clone())
                .into_arc_and_any(),
            Self::ReleaseGroup(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_release_group().as_ref().clone())
                .into_arc_and_any(),
            Self::Work(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_work().as_ref().clone())
                .into_arc_and_any(),
        }
    }

    pub fn as_kind(&self) -> MusicbrainzEntityKind {
        match self {
            Self::Artist(val) => val.as_kind(),
            Self::Recording(val) => val.as_kind(),
            Self::Release(val) => val.as_kind(),
            Self::ReleaseGroup(val) => val.as_kind(),
            Self::Work(val) => val.as_kind(),
        }
    }

    pub fn get_mbidspe(&self) -> AnyMBIDType<PrimaryID> {
        match self {
            Self::Artist(val) => val.get_mbidspe().into(),
            Self::Recording(val) => val.get_mbidspe().into(),
            Self::Release(val) => val.get_mbidspe().into(),
            Self::ReleaseGroup(val) => val.get_mbidspe().into(),
            Self::Work(val) => val.get_mbidspe().into(),
        }
    }

    pub fn into_any(self: Arc<Self>) -> Self {
        match self.deref().clone() {
            Self::Artist(val) => val.into_any(),
            Self::Recording(val) => val.into_any(),
            Self::Release(val) => val.into_any(),
            Self::ReleaseGroup(val) => val.into_any(),
            Self::Work(val) => val.into_any(),
        }
    }

    pub fn get_mbid(&self) -> MBID {
        match self {
            Self::Artist(val) => val.get_mbid().into(),
            Self::ReleaseGroup(val) => val.get_mbid().into(),
            Self::Release(val) => val.get_mbid().into(),
            Self::Recording(val) => val.get_mbid().into(),
            Self::Work(val) => val.get_mbid().into(),
        }
    }
}
