use serde::Deserialize;
use serde::Serialize;

use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct MBIDSpe<T: IdEntityType, S: IdAliasState> {
    id: String,

    _entity_type: PhantomData<T>,
    _state: PhantomData<S>,
}

pub trait IdEntityType: Clone {}
impl IdEntityType for Artist {}
impl IdEntityType for Recording {}

// Id state
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NaiveID;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrimaryID;

pub trait IdAliasState: Clone {}
impl IdAliasState for NaiveID {}
impl IdAliasState for PrimaryID {}

/// `MBIDSpe`'s Common Methods that change depending on the type and state
pub trait MBIDSpeMethods<T: IdEntityType, S: IdAliasState> {}

/// `MBIDSpe`'s Common Methods that change depending on the type only
pub trait MBIDSpeTypeMethods<T: IdEntityType> {}

/// `MBIDSpe`'s Common Methods that change depending on the state only
pub trait MBIDSpeStateMethods<S: IdAliasState> {}

impl<T: IdEntityType, S: IdAliasState> Deref for MBIDSpe<T, S> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T, S> From<String> for MBIDSpe<T, S>
where
    T: IdEntityType,
    S: IdAliasState,
{
    fn from(value: String) -> Self {
        Self {
            id: value,
            _entity_type: PhantomData,
            _state: PhantomData,
        }
    }
}
