use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;

use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

use super::is_musicbrainz_id::IsMusicbrainzID;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    pub(super) id: String,

    _entity_type: PhantomData<T>,
    _state: PhantomData<S>,
}

// Id state
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Hash, Clone)]
pub struct NaiveID;
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Hash, Clone)]
pub struct PrimaryID;

pub trait IdAliasState: Sync + Send {}
impl IdAliasState for NaiveID {}
impl IdAliasState for PrimaryID {}

pub type NaiveMBID<T> = MBIDSpe<T, NaiveID>;
pub type PrimaryMBID<T> = MBIDSpe<T, PrimaryID>;

impl<T, S> Deref for MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T, S> From<String> for MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn from(value: String) -> Self {
        Self {
            id: value,
            _entity_type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<T, S> Display for MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T, S> MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<T, S> Hash for MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
