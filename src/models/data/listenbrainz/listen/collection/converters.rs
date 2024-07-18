use std::sync::Arc;

use futures::future;
use futures::stream;
use futures::StreamExt;
use futures::TryStreamExt;
use itertools::Itertools;

use crate::core::display::progress_bar::ProgressBarCli;
use crate::core::entity_traits::mbid::is_cached_mbid::IsCachedMBID;
use crate::models::data::listenbrainz::listen::primary_listen::PrimaryListen;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::mapped_primary_collection::PrimaryListenCollection;
use super::ListenCollection;

impl ListenCollection {
    pub async fn get_listened_recordings_mbids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        let naive_recordings = self.get_listened_recordings_mbids_unchecked();

        let progress = ProgressBarCli::new(
            naive_recordings.len() as u64,
            Some("Getting listened recordings"),
        );
        let stream = progress.wrap_stream(stream::iter(naive_recordings));

        let primaries: Vec<RecordingMBID> = stream
            .map(|recording| async move { recording.get_primary_alias().await })
            .buffer_unordered(20)
            .try_collect()
            .await?;
        Ok(primaries.into_iter().unique().collect_vec())
    }

    /// Get the raw listen MBIDs without checking their real ID.
    pub fn get_listened_recordings_mbids_unchecked(&self) -> Vec<RecordingMBID> {
        self.iter()
            .filter_map(|listen| listen.get_naive_recording_mbid())
            .unique()
            .collect()
    }

    pub async fn try_into_mapped_primary(self) -> color_eyre::Result<PrimaryListenCollection> {
        let pg = Arc::new(ProgressBarCli::new(
            self.len() as u64,
            Some("Loading listened recordings"),
        ));

        stream::iter(self.into_iter())
            .filter_map(|listen| async { PrimaryListen::from_listen(listen).await })
            .map(future::ready)
            .inspect(|_| pg.inc(1_u64))
            .buffer_unordered(20)
            .map_ok(Arc::new)
            .try_collect()
            .await
    }
}
