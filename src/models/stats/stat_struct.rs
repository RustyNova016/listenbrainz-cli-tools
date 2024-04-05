use std::rc::Rc;

use crate::models::{cli::stats::GroupByTarget, data::listens::UserListen};

pub struct EntityStats {
    mbid: String,
    entity_type: GroupByTarget,
    listens: Vec<Rc<UserListen>>,
}

impl EntityStats {

    pub fn get_mbid(&self) -> &str {
        &self.mbid
    }

    pub fn new(mbid: String, entity_type: GroupByTarget) -> Self {
        Self {
            mbid,
            entity_type,
            listens: Vec::new(),
        }
    }
}

pub trait StatStruct {
    fn push(&mut self, value: Rc<UserListen>);

    fn get_mbid(&self) -> &str;

    fn new(mbid: String) -> Self;
}
