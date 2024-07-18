use crate::core::data_structures::table_map::table_index::TableItem;
use std::hash::Hash;

use super::ListensWithEntityMap;

impl<K, E> Default for ListensWithEntityMap<K, E>
where
    K: Eq + Hash,
    E: TableItem<K>,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
