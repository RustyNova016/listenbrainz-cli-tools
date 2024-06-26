use std::mem::discriminant;
use std::sync::Arc;

use derive_more::{From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::MBIDEnum;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::utils::println_cli_warn;

/// Any entity from Musicbrainz, with the same format as the official database
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IsVariant, Unwrap, From)]
pub enum AnyMusicBrainzEntity {
    Artist(Arc<Artist>),
    ReleaseGroup(Arc<ReleaseGroup>),
    Release(Arc<Release>),
    Recording(Arc<Recording>),
    Work(Arc<Work>),
}

impl AnyMusicBrainzEntity {
    pub async fn update_cache(&self) -> color_eyre::Result<()> {
        match self.clone() {
            Self::ReleaseGroup(val) => MUSICBRAINZ_DATABASE.release_groups().update(val).await?,
            Self::Release(val) => MUSICBRAINZ_DATABASE.releases().update(val).await?,
            Self::Recording(val) => MUSICBRAINZ_DATABASE.recordings().update(val).await?,
            Self::Work(val) => MUSICBRAINZ_DATABASE.works().update(val).await?,
            Self::Artist(val) => MUSICBRAINZ_DATABASE.artists().update(val).await?,
        }

        Ok(())
    }
}

impl HasMBID<MBIDEnum> for AnyMusicBrainzEntity {
    fn get_mbid(&self) -> MBIDEnum {
        match self {
            Self::Artist(val) => val.get_mbid().into(),
            Self::ReleaseGroup(val) => val.get_mbid().into(),
            Self::Release(val) => val.get_mbid().into(),
            Self::Recording(val) => val.get_mbid().into(),
            Self::Work(val) => val.get_mbid().into(),
        }
    }
}

impl Updatable for AnyMusicBrainzEntity {
    fn update(self, newer: Self) -> Self {
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
                .into_generic(),
            Self::Recording(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_recording().as_ref().clone())
                .into_generic(),
            Self::Release(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_release().as_ref().clone())
                .into_generic(),
            Self::ReleaseGroup(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_release_group().as_ref().clone())
                .into_generic(),
            Self::Work(val) => val
                .as_ref()
                .clone()
                .update(newer.unwrap_work().as_ref().clone())
                .into_generic(),
        }
    }
}

impl From<Artist> for AnyMusicBrainzEntity {
    fn from(value: Artist) -> Self {
        Self::Artist(Arc::new(value))
    }
}

impl From<Recording> for AnyMusicBrainzEntity {
    fn from(value: Recording) -> Self {
        Self::Recording(Arc::new(value))
    }
}

impl From<Release> for AnyMusicBrainzEntity {
    fn from(value: Release) -> Self {
        Self::Release(Arc::new(value))
    }
}

impl From<ReleaseGroup> for AnyMusicBrainzEntity {
    fn from(value: ReleaseGroup) -> Self {
        Self::ReleaseGroup(Arc::new(value))
    }
}

impl From<Work> for AnyMusicBrainzEntity {
    fn from(value: Work) -> Self {
        Self::Work(Arc::new(value))
    }
}

impl<V> From<V> for AnyMusicBrainzEntity
where
    V: HasMBID,
{
    fn from(value: V) -> Self {
        match value.get_kind() {
            MusicbrainzEntityKind::Artist => Self::Artist(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MusicbrainzEntityKind {
    Artist,
    Recording,
    Release,
    ReleaseGroup,
    Work,
}
