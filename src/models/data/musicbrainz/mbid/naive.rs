use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

use super::generic_mbid::NaiveMBID;
use super::is_musicbrainz_id::IsMusicbrainzID;

impl<T> NaiveMBID<T>
where
    T: IsMusicbrainzEntity,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
}
