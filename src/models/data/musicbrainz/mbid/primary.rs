use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

use super::generic_mbid::NaiveMBID;
use super::generic_mbid::PrimaryMBID;
use super::is_musicbrainz_id::IsMusicbrainzID;

impl<T> PrimaryMBID<T>
where
    T: IsMusicbrainzEntity,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    pub fn into_naive(self) -> NaiveMBID<T> {
        self.into()
    }
}

impl<T> From<PrimaryMBID<T>> for NaiveMBID<T>
where
    T: IsMusicbrainzEntity,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn from(value: PrimaryMBID<T>) -> Self {
        Self::from(value.id().to_string())
    }
}
