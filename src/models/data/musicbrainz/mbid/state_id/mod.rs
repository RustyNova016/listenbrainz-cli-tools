pub mod collection;
pub mod any;
pub mod converters;
pub mod state;
pub mod traits;

use std::marker::PhantomData;

use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;

pub mod impls;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Getters)]
pub struct MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    pub(super) id: String,

    _entity_type: PhantomData<T>,
    _state: PhantomData<S>,
}

// Typestate markers

pub trait MBIDState {}
