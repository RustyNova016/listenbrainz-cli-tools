use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;

use crate::models::data::listens::{collection::UserListenCollection, UserListen};

use super::{stat_struct::StatStruct, StatSorter};

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

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RecordingStatsSorter {
    listens: HashMap<String, UserListenCollection>,
}

impl StatSorter for RecordingStatsSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, UserListenCollection> {
        &mut self.listens
    }

    fn push(&mut self, value: Rc<UserListen>, mb_client: &mut crate::models::api::musicbrainz::MusicBrainzAPI) {
        if let Some(mapping_info) = value.mapping_data {
            self.get_mut(&mapping_info.recording_mbid).push(value)
        }
    }
    
    fn into_vec(self) -> Vec<(String, UserListenCollection)> {
        self.listens.into_iter().collect_vec()
    }
}
