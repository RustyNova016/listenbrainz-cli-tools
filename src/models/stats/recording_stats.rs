use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;

use crate::models::data::listens::{collection::UserListenCollection, UserListen};

use super::StatSorter;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RecordingStatsSorter {
    listens: HashMap<String, UserListenCollection>,
}

impl RecordingStatsSorter {
    pub fn new() -> Self {
        Self {
            listens: HashMap::new(),
        }
    }
}

impl StatSorter for RecordingStatsSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, UserListenCollection> {
        &mut self.listens
    }

    fn push(&mut self, value: Arc<UserListen>) {
        if let Some(mapping_info) = &value.mapping_data {
            self.get_mut(&mapping_info.recording_mbid).push(value)
        }
    }

    fn into_vec(self) -> Vec<(String, UserListenCollection)> {
        self.listens.into_iter().collect_vec()
    }
}
