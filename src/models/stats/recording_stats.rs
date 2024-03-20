use std::rc::Rc;

use crate::models::data::listens::UserListen;

use super::stat_struct::StatStruct;

pub struct RecordingStats {
    mbid: String,
    listens: Vec<Rc<UserListen>>,
}

impl StatStruct for RecordingStats {
    fn push(&mut self, value: Rc<UserListen>) {
        todo!()
    }

    fn get_mbid(&self) -> &str {
        todo!()
    }

    fn new(mbid: String) -> Self {
        todo!()
    }
}
