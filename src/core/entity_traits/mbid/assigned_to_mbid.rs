use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::mbid::MBID;

/// Trait for all the entities that are assigned to an MBID.
pub trait AssignedToMBID {
    fn get_assigned_mbid(&self) -> MBID;
}

pub trait AssignedToMBIDType<I, T> where I: IsMbid<T>, T: HasMBID<I> {
    fn get_assigned_mbid_type(&self) -> I;
}

//impl<I, T, A> AssignedToMBID for A where I: IsMbid<T>, T: HasMBID<I>, A: AssignedToMBIDType<I, T> {
//    fn get_assigned_mbid(&self) -> MBID {
//        self.get_assigned_mbid_type().into_mbid()
//    }
//}