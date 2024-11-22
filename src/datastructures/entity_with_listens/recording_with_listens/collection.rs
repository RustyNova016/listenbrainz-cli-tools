use std::collections::HashMap;

use futures::stream;
use futures::Stream;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::Decimal;

use crate::datastructures::listen_collection::traits::ListenCollectionLike;

use super::RecordingWithListens;

/// An `HashMap` containing `RecordingWithListens`, indexed on the Recording's ID
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RecordingWithListensCollection(pub HashMap<i64, RecordingWithListens>);

impl RecordingWithListensCollection {
    pub fn iter_recordings(&self) -> impl Iterator<Item = &Recording> {
        self.0.values().map(|r| r.recording())
    }

    pub fn into_iter_recordings(self) -> impl Iterator<Item = Recording> {
        self.0.into_values().map(|r| r.recording)
    }

    pub fn into_values(self) -> impl Iterator<Item = RecordingWithListens> {
        self.0.into_values()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_by_id(&self, id: i64) -> Option<&RecordingWithListens> {
        self.0.get(&id)
    }

    pub fn get_by_mbid(&self, mbid: &str) -> Option<&RecordingWithListens> {
        self.0.values().find(|r| r.recording().mbid == mbid)
    }

    /// Return the ratio of listens being from a recording
    pub fn get_listen_ratio(&self, recording: &Recording) -> Decimal {
        let recording_listen_count = self
            .get_by_id(recording.id)
            .map(|r| r.listen_count())
            .unwrap_or(0);

        Decimal::new(recording_listen_count.try_into().unwrap(), 0)
            / Decimal::new(self.listen_count().try_into().unwrap(), 0)
    }

    pub fn into_values_stream(self) -> impl Stream<Item = RecordingWithListens> {
        stream::iter(self.0.into_values())
    }
}

impl ListenCollectionLike for RecordingWithListensCollection {
    fn iter_listens(
        &self,
    ) -> impl Iterator<Item = &musicbrainz_db_lite::models::listenbrainz::listen::Listen> {
        self.0.values().flat_map(|l| l.iter_listens())
    }
}

impl From<HashMap<i64, RecordingWithListens>> for RecordingWithListensCollection {
    fn from(value: HashMap<i64, RecordingWithListens>) -> Self {
        Self(value)
    }
}

impl From<RecordingWithListensCollection> for HashMap<i64, RecordingWithListens> {
    fn from(value: RecordingWithListensCollection) -> Self {
        value.0
    }
}
