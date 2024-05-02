use std::fmt::Display;

pub trait HasID {
    fn get_id(&self) -> String;
}
