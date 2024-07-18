use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveMBID;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;

use super::is_musicbrainz_entity::IsMusicbrainzEntity;
use extend::ext;
use itertools::Itertools;

#[ext(name = MBEntityCollection)]
pub impl<T> Vec<T>
where
    T: IsMusicbrainzEntity,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn to_ids(&self) -> Vec<PrimaryMBID<T>> {
        self.iter().map(|ent| ent.get_mbidspe()).collect_vec()
    }
}
