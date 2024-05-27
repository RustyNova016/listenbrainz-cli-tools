use std::mem::discriminant;

use derive_more::{From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::utils::println_cli_warn;

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
            Self::ReleaseGroup(val) => MUSICBRAINZ_DATABASE.release_groups().update(val).await?,
            Self::Release(val) => MUSICBRAINZ_DATABASE.releases().update(val).await?,
            Self::Recording(val) => MUSICBRAINZ_DATABASE.recordings().update(val).await?,
            Self::Work(val) => MUSICBRAINZ_DATABASE.works().update(val).await?,
            Self::Artist(val) => MUSICBRAINZ_DATABASE.artists().update(val).await?,
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

impl Updatable for MusicBrainzEntity {
    fn update(self, newer: Self) -> Self {
        // Check if both are the same variant
        if discriminant(&self) != discriminant(&newer) {
            // No big deal. But worth mentioning
            // TODO: Debug only warning
            println_cli_warn("Tried to update entity type with mismatched entity type".to_string());
            return self;
        }

        match self {
            Self::Artist(val) => val.update(newer.unwrap_artist()).into(),
            Self::Recording(val) => val.update(newer.unwrap_recording()).into(),
            Self::Release(val) => val.update(newer.unwrap_release()).into(),
            Self::ReleaseGroup(val) => val.update(newer.unwrap_release_group()).into(),
            Self::Work(val) => val.update(newer.unwrap_work()).into(),
        }
    }
}
