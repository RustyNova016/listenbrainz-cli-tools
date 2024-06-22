use std::borrow::Borrow;

use extend::ext;
use itertools::Itertools;

use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::mbid::IsMbid;

#[ext]
pub impl<AT: Borrow<T>, T: HasMBID<K>, K: IsMbid<T>> Vec<AT> {
    fn into_mbids(self) -> Vec<K> {
        self.into_iter()
            .map(|data| data.borrow().get_mbid())
            .collect_vec()
    }
}
