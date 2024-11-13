use core::future::ready;

use futures::{Stream, StreamExt};

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;

pub fn timeout_filter(
    listens: impl StreamExt<Item = RecordingWithListens>,
) -> impl Stream<Item = RecordingWithListens> {
    let timeouts = RecordingTimeoutConfig::get_timed_out_recordings()
        .expect("Couldn't fetch the timeout config");

    listens.filter(move |r| ready(!timeouts.iter().any(|t| **t == r.recording().mbid)))
}
