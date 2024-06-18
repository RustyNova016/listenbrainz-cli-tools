use std::sync::Arc;

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::listen_spe::ListenSpe;
use super::listen_spe::MappedNaive;
use super::listen_spe::Unmapped;
use chrono::DateTime;
use chrono::Utc;
use derive_more::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Unwrap, IsVariant, Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ListenMappingState {
    Unmapped(Arc<ListenSpe<Unmapped>>),
    Mapped(Arc<ListenSpe<MappedNaive>>),
}

impl ListenMappingState {
    pub async fn get_primary_recording_id(&self) -> color_eyre::Result<Option<RecordingMBID>> {
        match self {
            Self::Mapped(val) => val.get_recording_mbid().await.map(Some),
            Self::Unmapped(_) => Ok(None),
        }
    }

    pub fn get_listened_at(&self) -> &DateTime<Utc> {
        match self {
            Self::Mapped(val) => val.get_listened_at(),
            Self::Unmapped(val) => val.get_listened_at(),
        }
    }

    pub fn as_mapped(&self) -> Option<&Arc<ListenSpe<MappedNaive>>> {
        match self {
            Self::Mapped(val) => Some(val),
            _ => None,
        }
    }
}
