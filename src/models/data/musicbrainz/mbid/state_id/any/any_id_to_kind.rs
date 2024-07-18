use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::error::Error;

use super::any_entity::AnyEntityMBID;

pub trait AnyIdToKind<T, S>
where
    Self: Sized,
    T: MusicBrainzEntity,
    S: MBIDState,
{
    /// Try turning an `AnyEntityMBID` into self
    fn try_from_any(value: AnyEntityMBID<S>) -> Result<Self, Error>;
}
