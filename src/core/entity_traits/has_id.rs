use std::fmt::Display;

pub trait HasID<K: Display> {
    fn get_id(&self) -> K;
}
