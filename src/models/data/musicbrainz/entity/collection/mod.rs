use std::ops::Deref;
use std::sync::Arc;

use itertools::Itertools;

use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;

use super::traits::MusicBrainzEntity;

pub trait CollectionOfEntity<T>
where
    Self: Deref<Target = [Arc<T>]> + Sized,
    T: MusicBrainzEntity,
{
    fn get_mbids(self) -> Vec<PrimaryMBID<T>> {
        self.iter().map(|val| val.get_mbid()).collect_vec()
    }

    fn get_mbids_as_naive(self) -> Vec<NaiveMBID<T>> {
        self.iter()
            .map(|val| val.get_mbid().into_naive())
            .collect_vec()
    }
}

impl<T> CollectionOfEntity<T> for Vec<Arc<T>> where T: MusicBrainzEntity {}
