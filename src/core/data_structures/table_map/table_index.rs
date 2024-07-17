use std::hash::Hash;

/// Represent an element in a [`TableMap`]
pub trait TableItem<K: Eq + Hash> {
    /// Return the key of the element
    fn get_key(&self) -> K;
}

// impl<K, V, Wrapper> TableItem<K> for Wrapper
// where
//     Wrapper: Deref<Target = V>,
//     V: TableItem<K>,
//     K: Eq + Hash,
// {
//     fn get_key(&self) -> K {
//         self.deref().get_key()
//     }
// }
