use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;

use color_eyre::{eyre::Ok, Result};

use crate::models::data::listenbrainz::listen::{collection::ListenCollection, Listen};

use super::StatSorter;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RecordingStatsSorter {
    listens: HashMap<String, ListenCollection>,
}

impl RecordingStatsSorter {
    pub fn new() -> Self {
        Self {
            listens: HashMap::new(),
        }
    }
}

impl StatSorter for RecordingStatsSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, ListenCollection> {
        &mut self.listens
    }

    fn push(&mut self, value: Arc<Listen>) -> Result<()> {
        if let Some(mapping_info) = &value.mapping_data {
            self.get_mut(&mapping_info.recording_mbid).push(value)
        }

        Ok(())
    }

    fn into_vec(self) -> Vec<(String, ListenCollection)> {
        self.listens.into_iter().collect_vec()
    }
}
