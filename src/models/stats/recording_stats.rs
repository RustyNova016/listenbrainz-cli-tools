use std::collections::HashMap;
use std::sync::Arc;

use chashmap::CHashMap;
use itertools::Itertools;

use color_eyre::eyre::Ok;
use color_eyre::Result;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;

use super::generic_statistic_holder::GenericStatisticHolder;
use super::{StatSorter, StatisticSorter};

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

    async fn push(&mut self, value: Arc<Listen>) -> Result<()> {
        if let Some(mapping_info) = &value.mapping_data {
            self.get_mut(&mapping_info.recording_mbid).push(value)
        }

        Ok(())
    }

    fn into_vec(self) -> Vec<(String, ListenCollection)> {
        self.listens.into_iter().collect_vec()
    }
}

pub struct RecordingStatisticSorter {
    data: CHashMap<String, Arc<GenericStatisticHolder<String>>>
}

impl StatisticSorter<String, GenericStatisticHolder<String>> for RecordingStatisticSorter {
    fn insert_listen(&self, listen: Arc<Listen>) -> impl std::future::Future<Output = Result<()>> {
        todo!()
    }

    fn get(&self, key: &String) -> Arc<GenericStatisticHolder<String>> {
        let collection = self.data.get(key);

        if let Some(collection) = collection {
            return collection.clone();
        }

        self.data.insert(key.to_string(), Arc::new(ArtistStatisticHolder::create(key.to_string())));
        self.data.get(key).map(|collection| collection.clone()).expect("Couldn't retrieve inserted collection")
    }
}