use std::hash::Hash;

use crate::core::data_structures::table_map::table_index::TableItem;

use super::ListensWithEntity;

impl<K, E> TableItem<K> for ListensWithEntity<E>
where
    E: TableItem<K>,
    K: Eq + Hash,
{
    fn get_key(&self) -> K {
        self.entity.get_key()
    }
}
