use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

use super::generic_mbid::NaiveMBID;
use super::generic_mbid::PrimaryMBID;
use super::is_musicbrainz_id::IsMusicbrainzID;

impl<T> PrimaryMBID<T>
where
    T: IsMusicbrainzEntity,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
}

// impl<T> From<PrimaryMBID<T>> for NaiveMBID<T>
// where
//     T: IsMusicbrainzEntity,
//     Self: IsMusicbrainzID<T>,
// {
//     fn from(value: PrimaryMBID<T>) -> Self {
//         Self::from(value.id().to_string())
//     }
// }
