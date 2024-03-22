use std::rc::Rc;

use crate::models::data::listens::UserListen;

use super::stat_struct::StatStruct;

pub struct RecordingStats {
    mbid: String,
    listens: Vec<Rc<UserListen>>,
}

impl StatStruct for RecordingStats {
    fn push(&mut self, value: Rc<UserListen>) {
        if value.is_mapped_to_recording(&self.mbid) {
            self.push(value)
        }
    }

    fn get_mbid(&self) -> &str {
        &self.mbid
    }

    fn new(mbid: String) -> Self {
        Self {
            mbid,
            listens: Vec::new(),
        }
    }
}

pub struct RecordingStatsSorter {
    mbid: String,
}
