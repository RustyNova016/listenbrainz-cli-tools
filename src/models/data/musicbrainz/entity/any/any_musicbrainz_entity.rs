use std::mem::discriminant;
use std::ops::Deref;
use std::sync::Arc;

use derive_more::{From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::state_id::any::any_entity::AnyEntityMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryIDState;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;
use crate::utils::println_cli_warn;

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
            Self::ReleaseGroup(val) => {
                MUSICBRAINZ_DATABASE_LEGACY
                    .release_groups()
                    .update(val)
                    .await?;
            }
            Self::Release(val) => MUSICBRAINZ_DATABASE_LEGACY.releases().update(val).await?,
            Self::Recording(val) => MUSICBRAINZ_DATABASE_LEGACY.recordings().update(val).await?,
            Self::Work(val) => MUSICBRAINZ_DATABASE_LEGACY.works().update(val).await?,
            Self::Artist(val) => MUSICBRAINZ_DATABASE_LEGACY.artists().update(val).await?,
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
                .incremental_update(newer.unwrap_artist().as_ref().clone())
                .into_arc_and_any(),
            Self::Recording(val) => val
                .as_ref()
                .clone()
                .incremental_update(newer.unwrap_recording().as_ref().clone())
                .into_arc_and_any(),
            Self::Release(val) => val
                .as_ref()
                .clone()
                .incremental_update(newer.unwrap_release().as_ref().clone())
                .into_arc_and_any(),
            Self::ReleaseGroup(val) => val
                .as_ref()
                .clone()
                .incremental_update(newer.unwrap_release_group().as_ref().clone())
                .into_arc_and_any(),
            Self::Work(val) => val
                .as_ref()
                .clone()
                .incremental_update(newer.unwrap_work().as_ref().clone())
                .into_arc_and_any(),
        }
    }

    pub fn get_kind(&self) -> MusicbrainzEntityKind {
        match self {
            Self::Artist(_) => MusicbrainzEntityKind::Artist,
            Self::Recording(_) => MusicbrainzEntityKind::Recording,
            Self::Release(_) => MusicbrainzEntityKind::Release,
            Self::ReleaseGroup(_) => MusicbrainzEntityKind::ReleaseGroup,
            Self::Work(_) => MusicbrainzEntityKind::Work,
        }
    }

    pub fn get_mbidspe(&self) -> AnyEntityMBID<PrimaryIDState> {
        match self {
            Self::Artist(val) => val.get_mbid().into(),
            Self::Recording(val) => val.get_mbid().into(),
            Self::Release(val) => val.get_mbid().into(),
            Self::ReleaseGroup(val) => val.get_mbid().into(),
            Self::Work(val) => val.get_mbid().into(),
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
}
