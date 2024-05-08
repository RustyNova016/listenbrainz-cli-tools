use super::Listen;
use crate::models::cli::common::SortListensBy;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

/// Wrapper for a vector of listens
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListenCollection {
    data: Vec<Arc<Listen>>,
}

impl ListenCollection {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_mapped_listens(&self) -> Self {
        self.data
            .iter()
            .filter(|element| element.is_mapped())
            .cloned()
            .collect()
    }

    /// Returns the latest listen in the collection.
    pub fn get_latest_listen(&self) -> Option<Arc<Listen>> {
        self.data
            .iter()
            .max_by_key(|listen| listen.listened_at)
            .cloned()
    }

    /// Returns all the unmapped listens
    pub fn get_unmapped_listens(&self) -> Self {
        self.data
            .iter()
            .filter(|listen| !listen.is_mapped())
            .cloned()
            .collect()
    }

    /// Remove all the listens in between two dates.
    pub fn remove_timerange(
        &mut self,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        inclusive: bool,
    ) {
        self.data.retain(|listen| {
            if inclusive {
                listen.get_listened_at() < start || end < listen.get_listened_at()
            } else {
                listen.get_listened_at() <= start || end <= listen.get_listened_at()
            }
        });
    }

    /// Add a listen to the collection.
    pub fn push(&mut self, listen: Arc<Listen>) {
        self.data.push(listen);
    }

    pub fn sort_by_criteria(&mut self, sort: SortListensBy) {
        match sort {
            SortListensBy::Name => {
                let mut sorted = self.to_vec();
                sorted.sort_by_key(|recording| {
                    recording
                        .get_mapping_data()
                        .as_ref()
                        .map(|data| data.recording_name.clone())
                        .unwrap_or(recording.get_messybrain_data().track_name.clone())
                });
                *self = Self { data: sorted }
            }

            SortListensBy::OldestListen => {
                let mut sorted = self.to_vec();
                sorted.sort_by_key(|recording| recording.listened_at);
                *self = Self { data: sorted }
            }

            SortListensBy::None => {}
        }
    }

    pub fn has_recording(&self, id: &str) -> bool {
        self.iter().any(|listen| {
            listen
                .get_mapping_data()
                .as_ref()
                .is_some_and(|mapping| mapping.recording_mbid == id)
        })
    }
}

impl Deref for ListenCollection {
    type Target = Vec<Arc<Listen>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl FromIterator<Arc<Listen>> for ListenCollection {
    fn from_iter<T: IntoIterator<Item = Arc<Listen>>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for ListenCollection {
    type Item = Arc<Listen>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Default for ListenCollection {
    fn default() -> Self {
        Self::new()
    }
}
