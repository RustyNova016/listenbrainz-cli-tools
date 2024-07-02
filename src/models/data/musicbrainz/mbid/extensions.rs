use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::mbid::IsMbid;
use extend::ext;
use itertools::Itertools;

#[ext]
pub impl<T: HasMBID<K>, K: IsMbid<T>> Vec<T> {
    fn into_mbids(self) -> Vec<K> {
        self.into_iter().map(|data| data.get_mbid()).collect_vec()
    }
}
