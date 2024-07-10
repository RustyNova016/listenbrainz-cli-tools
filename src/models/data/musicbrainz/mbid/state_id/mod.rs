pub mod state;
pub mod traits;

use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

pub mod impls;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct MBIDWithState<T, S>
where
    T: MusicBrainzEntity + ?Sized,
    S: MBIDState + ?Sized,
{
    pub(super) id: String,

    _entity_type: PhantomData<T>,
    _state: PhantomData<S>,
}

// Typestate markers

/// Marker trait for an MusicBrainz entity
pub trait MusicBrainzEntity {}

pub trait MBIDState {}
