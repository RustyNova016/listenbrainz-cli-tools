use std::mem::discriminant;
use std::ops::Deref;
use std::sync::Arc;

use derive_more::{From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mbid::HasMBID;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;
use crate::utils::println_cli_warn;

use super::entity::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity::entity_kind::MusicbrainzEntityKind;
use super::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use super::mbid::generic_mbid::MBIDSpe;
use super::mbid::generic_mbid::PrimaryID;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IsVariant, Unwrap, From)]
pub enum MusicBrainzEntity {
    Artist(Artist),
    ReleaseGroup(ReleaseGroup),
    Release(Release),
    Recording(Recording),
    Work(Work),
}

impl MusicBrainzEntity {
    pub async fn save_to_cache(&self) -> color_eyre::Result<()> {
        match self {
            Self::ReleaseGroup(val) => MUSICBRAINZ_DATABASE_LEGACY.release_groups().update(val).await?,
            Self::Release(val) => MUSICBRAINZ_DATABASE_LEGACY.releases().update(val).await?,
            Self::Recording(val) => MUSICBRAINZ_DATABASE_LEGACY.recordings().update(val).await?,
            Self::Work(val) => MUSICBRAINZ_DATABASE_LEGACY.works().update(val).await?,
            Self::Artist(val) => MUSICBRAINZ_DATABASE_LEGACY.artists().update(val).await?,
        }

        Ok(())
    }
}

impl HasMBID<MBID> for MusicBrainzEntity {
    fn get_mbid(&self) -> MBID {
        match self {
            Self::Artist(val) => val.get_mbid().into(),
            Self::ReleaseGroup(val) => val.get_mbid().into(),
            Self::Release(val) => val.get_mbid().into(),
            Self::Recording(val) => val.get_mbid().into(),
            Self::Work(val) => val.get_mbid().into(),
        }
    }
}

impl IsMusicbrainzEntity for MusicBrainzEntity {
    fn partial_update(self, newer: Self) -> Self {
        // Check if both are the same variant
        if discriminant(&self) != discriminant(&newer) {
            // No big deal. But worth mentioning
            // TODO: Debug only warning
            println_cli_warn("Tried to update entity type with mismatched entity type".to_string());
            return self;
        }

        match self {
            Self::Artist(val) => val.partial_update(newer.unwrap_artist()).into(),
            Self::Recording(val) => val.partial_update(newer.unwrap_recording()).into(),
            Self::Release(val) => val.partial_update(newer.unwrap_release()).into(),
            Self::ReleaseGroup(val) => val.partial_update(newer.unwrap_release_group()).into(),
            Self::Work(val) => val.partial_update(newer.unwrap_work()).into(),
        }
    }

    fn as_kind(&self) -> MusicbrainzEntityKind {
        match self {
            Self::Artist(val) => val.as_kind(),
            Self::Recording(val) => val.as_kind(),
            Self::Release(val) => val.as_kind(),
            Self::ReleaseGroup(val) => val.as_kind(),
            Self::Work(val) => val.as_kind(),
        }
    }

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID> {
        let id: String = match self {
            Self::Artist(val) => val.get_mbidspe().to_string(),
            Self::Recording(val) => val.get_mbidspe().to_string(),
            Self::Release(val) => val.get_mbidspe().to_string(),
            Self::ReleaseGroup(val) => val.get_mbidspe().to_string(),
            Self::Work(val) => val.get_mbidspe().to_string(),
        };

        MBIDSpe::from(id)
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        match self.deref().clone() {
            Self::Artist(val) => val.into_arc_and_any(),
            Self::Recording(val) => val.into_arc_and_any(),
            Self::Release(val) => val.into_arc_and_any(),
            Self::ReleaseGroup(val) => val.into_arc_and_any(),
            Self::Work(val) => val.into_arc_and_any(),
        }
    }
}
