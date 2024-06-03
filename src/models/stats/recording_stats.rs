use std::collections::HashMap;
use std::sync::Arc;

use chashmap::CHashMap;
use itertools::Itertools;

use color_eyre::eyre::Ok;
use color_eyre::Result;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;

use crate::core::statistics::generic_statistic_holder::GenericStatisticHolder;
use crate::core::statistics::stat_sorter::StatSorter;
use crate::core::statistics::statistic_holder::StatisticHolder;
use crate::core::statistics::statistic_sorter_trait::StatisticSorter;

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

    fn into_vec(self) -> Vec<(String, ListenCollection)> {
        self.listens.into_iter().collect_vec()
    }

    async fn push(&mut self, value: Arc<Listen>) -> Result<()> {
        if let Some(mapping_info) = &value.mapping_data {
            self.get_mut(&mapping_info.recording_mbid).push(value);
        }

        Ok(())
    }
}

pub struct RecordingStatisticSorter {
    data: CHashMap<String, Arc<GenericStatisticHolder<String>>>,
}

impl StatisticSorter<String, GenericStatisticHolder<String>> for RecordingStatisticSorter {
    async fn insert_listen(&self, listen: Arc<Listen>) -> Result<()> {
        let Some(recording_mbid) = listen.get_recording_mbid_as_string() else {
            return Ok(());
        };

        let holder = self.get(recording_mbid);

        holder.insert_listen(listen).await?;
        Ok(())
    }

    fn get(&self, key: &String) -> Arc<GenericStatisticHolder<String>> {
        let collection = self.data.get(key);

        if let Some(collection) = collection {
            return collection.clone();
        }

        self.data.insert(
            key.to_string(),
            Arc::new(GenericStatisticHolder::create(key.to_string())),
        );
        self.data
            .get(key)
            .map(|collection| collection.clone())
            .expect("Couldn't retrieve inserted collection")
    }

    fn into_vec(self) -> Vec<(String, Arc<GenericStatisticHolder<String>>)> {
        self.data.into_iter().collect_vec()
    }
}
