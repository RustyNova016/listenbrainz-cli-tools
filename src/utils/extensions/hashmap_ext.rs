use std::collections::HashMap;
use std::hash::Hash;

use extend::ext;

#[ext]
pub impl<K, V> HashMap<K, V> {
    /// [`get`](HashMap::get) an item, and if not found, [`insert`](Hashmap::insert) a new one, then return it
    fn get_or_insert_new(&mut self, key: &K, new: V) -> &V
    where
        K: Eq + Hash + Clone,
    {
        if self.contains_key(key) {
            return self
                .get(key)
                .expect("The key exist, so it's safe to unwrap");
        }

        self.insert(key.clone(), new);
        self.get(key).expect("Couldn't get the inserted value. This is impossible as K is Eq, and thus is assured to be found")
    }

    /// [`get`](HashMap::get) an item, and if not found, [`insert`](Hashmap::insert) a new one from a function, then return it
    fn get_or_then_insert_new<F>(&mut self, key: &K, f: F) -> &V
    where
        K: Eq + Hash + Clone,
        F: FnOnce() -> V,
    {
        if self.contains_key(key) {
            return self
                .get(key)
                .expect("The key exist, so it's safe to unwrap");
        }

        self.insert(key.clone(), f());
        self.get(key).expect("Couldn't get the inserted value. This is impossible as K is Eq, and thus is assured to be found")
    }
}
