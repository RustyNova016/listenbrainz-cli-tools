use std::sync::Arc;
use super::listen_spe::ListenSpe;
use super::listen_spe::MappedNaive;
use super::listen_spe::MappedPrimary;
use super::listen_spe::Unmapped;
use super::mapped_primary::MappedListen;
use chrono::DateTime;
use chrono::Utc;
use derive_more::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Unwrap, IsVariant, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, From)]
pub enum ListenMappingState {
    Unmapped(Arc<ListenSpe<Unmapped>>),
    MappedNaive(Arc<ListenSpe<MappedNaive>>),
    Mapped(Arc<MappedListen>),
}

impl ListenMappingState {
    pub fn listened_at(&self) -> &DateTime<Utc> {
        match self {
            Self::Mapped(val) => val.listened_at(),
            Self::Unmapped(val) => val.listened_at(),
            Self::MappedNaive(val) => val.listened_at(),
        }
    }

    pub fn as_mapped_naive(&self) -> Option<&Arc<ListenSpe<MappedNaive>>> {
        match self {
            Self::MappedNaive(val) => Some(val),
            _ => None,
        }
    }
}

impl From<ListenSpe<MappedPrimary>> for ListenMappingState {
    fn from(value: ListenSpe<MappedPrimary>) -> Self {
        Self::Mapped(Arc::new(value))
    }
}

impl From<ListenSpe<MappedNaive>> for ListenMappingState {
    fn from(value: ListenSpe<MappedNaive>) -> Self {
        Self::MappedNaive(Arc::new(value))
    }
}

impl From<ListenSpe<Unmapped>> for ListenMappingState {
    fn from(value: ListenSpe<Unmapped>) -> Self {
        Self::Unmapped(Arc::new(value))
    }
}