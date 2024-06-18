use std::sync::Arc;

use async_fn_stream::try_fn_stream;
use chrono::DateTime;
use chrono::Utc;
use extend::ext;
use futures::stream;
use futures::Stream;
use futures::StreamExt;
use futures::TryFutureExt;
use itertools::Itertools;

use crate::models::data::listenbrainz::listen::listen_spe::ListenSpe;
use crate::models::data::listenbrainz::listen::listen_spe::MappedNaive;
use crate::models::data::listenbrainz::listen::stream::convertion::SExt;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::extensions::future_ext::cStreamExt;
use futures::TryStreamExt;

use super::mapped_listen_collection::MappedListenCollection;

pub type MappedNaiveListensCollection = Vec<Arc<ListenSpe<MappedNaive>>>;

#[ext]
pub impl MappedNaiveListensCollection {
    // --- Methods to filter out data ---

    /// This filter out all the listens that are in the blacklist.
    ///
    /// # Integrity
    /// This function take naive recording ids. Make sure to check they are the primary ones
    fn filter_out_recordings_naive(&self, recordings: &[RecordingMBID]) -> Self {
        self.iter()
            .filter(|listen| !recordings.contains(&listen.get_legacy_recording_mbid()))
            .cloned()
            .collect_vec()
    }

    //async fn filter_out_recordings(&self, recordings: &[RecordingMBID]) -> Self {
    //    let self_copy = self.clone();
    //
    //    let stream = stream::iter(self)
    //        .map(|listen| async move {(listen, listen.get_recording_mbid().await)})
    //        .buffer_unordered(1)
    //        .filter_map(|(listen, id)| async move {
    //        match id {
    //            Err(val) => {return Some(Err(val));}
    //            Ok(val) => {
    //                if recordings.contains(&val) {
    //                    return Some(Ok(listen));
    //                }
    //                return None;
    //            }
    //        }
    //    });
    //
    //    let mut result = Vec::new();
    //
    //    while let Some(recording_id) = stream.next().await.transpose()? {
    //        result.push(recording_id);
    //    }
    //
    //    Ok(result)
    //}

    // Methods to retain data
    fn retain_ref_listened_after(&self, date: &DateTime<Utc>) -> Self {
        self.into_iter()
            .filter(|listen| listen.get_listened_at() > date)
            .cloned()
            .collect_vec()
    }

    // Convertion methods
    fn as_legacy_naive_recording_mbids(&self) -> Vec<RecordingMBID> {
        self.into_iter()
            .map(|listen| listen.get_legacy_recording_mbid())
            .collect_vec()
    }

    fn as_legacy_recording_mbid_stream(
        &self,
    ) -> impl Stream<Item = color_eyre::Result<RecordingMBID>> {
        stream::iter(self)
            .map(|listen| listen.get_recording_mbid())
            .buffer_unordered(20)
    }

    async fn as_legacy_recording_mbids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        let mut result = Vec::new();

        while let Some(recording_id) = self
            .as_legacy_recording_mbid_stream()
            .next()
            .await
            .transpose()?
        {
            result.push(recording_id);
        }

        Ok(result)
    }

    async fn into_primary(self) -> color_eyre::Result<MappedListenCollection> {
        stream::iter(self)
            .map(|listen| listen.as_ref().clone())
            .map(|a| a)
            .into_primary()
            .and_then(Arc::new)
            .buffer_unordered_non_future(50)
            .try_collect()
            .await
    }
}
