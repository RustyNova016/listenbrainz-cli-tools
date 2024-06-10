pub mod config;
mod converters;
pub mod filters;
pub mod listen_rate;
pub mod recording;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::models::cli::common::SortListensBy;
use crate::models::data::musicbrainz::mbid::mbid_merge::MBIDMerge;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::Listen;

pub mod stats;
mod underrated;

use derive_more::*;

/// Wrapper for a vector of listens
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut, IntoIterator)]
pub struct ListenCollection {
    #[deref]
    #[deref_mut]
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

    /// Returns the oldest listen in the collection.
    pub fn get_oldest_listen(&self) -> Option<Arc<Listen>> {
        self.data
            .iter()
            .min_by_key(|listen| listen.listened_at)
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
                        .unwrap_or_else(|| recording.get_messybrain_data().track_name.clone())
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

    /// Return the list of unique recordings ids that have been listened to.
    pub fn get_listened_recordings_mbids_naive(&self) -> Vec<RecordingMBID> {
        self.get_mapped_listens()
            .into_iter()
            .map(|listen| {
                listen
                    .get_mapping_data()
                    .as_ref()
                    .unwrap()
                    .recording_mbid
                    .clone()
            })
            .unique()
            .map_into()
            .collect_vec()
    }

    pub fn transmute_listens(self, merges: Vec<MBIDMerge<RecordingMBID>>) {
        let mut results = Vec::new();

        for listen in self.into_iter() {
            let Some(recording_string) = listen.get_recording_mbid_as_string() else {
                results.push(listen);
                continue;
            };

            let recording: RecordingMBID = recording_string.clone().into();

            let Some(merge) = merges.iter().find(|merge| merge.from() == &recording) else {
                results.push(listen);
                continue;
            };

            let mut new_listen = listen.as_ref().clone();
            new_listen.set_recording_mapping(merge.to().clone());
            results.push(Arc::new(new_listen));
        }
    }
}

impl FromIterator<Arc<Listen>> for ListenCollection {
    fn from_iter<T: IntoIterator<Item = Arc<Listen>>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect_vec(),
        }
    }
}

impl Default for ListenCollection {
    fn default() -> Self {
        Self::new()
    }
}
