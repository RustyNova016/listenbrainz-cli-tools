use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

#[derive(Debug, PartialEq, Eq)]
pub struct MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
{
    id: String,

    _entity_type: PhantomData<T>,
    _state: PhantomData<S>,
}

// Id state
pub struct NaiveID;
pub struct PrimaryID;

pub trait IdAliasState: Sync + Send {}
impl IdAliasState for NaiveID {}
impl IdAliasState for PrimaryID {}

impl<T: IsMusicbrainzEntity, S: IdAliasState> Deref for MBIDSpe<T, S> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T: IsMusicbrainzEntity, S: IdAliasState> From<String> for MBIDSpe<T, S> {
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
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
