use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::models::cli::common::SortSorterBy;
use chashmap::{CHashMap, ReadGuard};
use derive_new::new;
use itertools::Itertools;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;

#[derive(Debug, new)]
pub struct StatisticSorter {
    #[new(default)]
    listens: CHashMap<String, Mutex<ListenCollection>>,
}

impl StatisticSorter {
    fn get_or_create(&self, key: &str) -> ReadGuard<String, Mutex<ListenCollection>> {
        let inside = self.listens.get(key);

        if let Some(data) = inside {
            return data;
        }

        let new = Mutex::new(ListenCollection::default());
        self.listens.insert(key.to_string(), new);
        return self
            .listens
            .get(key)
            .expect("Failed to get element just inserted");
    }

    pub fn insert(&self, key: &str, listen: Arc<Listen>) {
        let element = self.get_or_create(key);
        let mut inner_coll = element.deref().lock().expect("Failed to get lock");

        inner_coll.push(listen);
    }

    pub fn into_sorted_vec(self, sort_by: SortSorterBy) -> Vec<(String, ListenCollection)> {
        let mut out = self.into_vec();

        match sort_by {
            SortSorterBy::Count => {
                out.sort_by_key(|item| item.1.len());
                out.reverse();
            }
            SortSorterBy::Name | SortSorterBy::Oldest => {} // TODO
        }

        out
    }

    pub fn into_vec(self) -> Vec<(String, ListenCollection)> {
        self.listens
            .into_iter()
            .map(|item| (item.0, item.1.into_inner().unwrap()))
            .collect_vec()
    }
}
