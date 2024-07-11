pub mod naive;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

use super::MBIDState;
use super::MBIDWithState;
use super::MusicBrainzEntity;

pub mod any_state;

impl<T, S> Deref for MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T, S> From<String> for MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    fn from(value: String) -> Self {
        Self {
            id: value,
            _entity_type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<T, S> Display for MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T, S> Hash for MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T, S> Clone for MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    fn clone(&self) -> Self {
        Self::from(self.id.clone())
    }
}
