use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use extend::ext;
use futures::stream;
use futures::Stream;
use futures::StreamExt;
use itertools::Itertools;

use crate::models::data::listenbrainz::listen::listen_spe::ListenSpe;
use crate::models::data::listenbrainz::listen::listen_spe::Mapped;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

pub type MappedListensCollection = Vec<Arc<ListenSpe<Mapped>>>;

#[ext]
pub impl MappedListensCollection {
    fn retain_ref_listened_after(&self, date: &DateTime<Utc>) -> Self {
        self.into_iter().filter(|listen| listen.get_listened_at() > date).cloned().collect_vec()
    }

    fn as_naive_recording_mbids(&self) -> Vec<RecordingMBID> {
        self.into_iter().map(|listen| listen.get_recording_mbid()).collect_vec()
    }

    fn as_recording_mbid_stream(&self) -> impl Stream<Item = color_eyre::Result<RecordingMBID>> {
        stream::iter(self).map(|listen| listen.get_primary_recording_id()).buffer_unordered(5)
    }

    async fn as_recording_mbids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        self.as_recording_mbid_stream().collect().await
    }
}