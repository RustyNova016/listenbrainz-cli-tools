use std::collections::hash_map::Entry;
use std::collections::HashMap;

use table_map::table_index::TableItem;

use std::hash::Hash;

pub mod table_map;

pub struct TableMap<K, V>(HashMap<K, V>)
where
    V: TableItem<K>,
    K: Eq + Hash;

impl<K, V> TableMap<K, V>
where
    V: TableItem<K>,
    K: Eq + Hash,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        self.0.get(k)
    }

    pub fn insert(&mut self, value: V) -> Option<V> {
        self.0.insert(value.get_key(), value)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.0.entry(key)
    }
}
