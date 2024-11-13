use core::future::ready;

use futures::{Stream, StreamExt};

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;

pub fn min_listen_filter(
    listens: impl StreamExt<Item = RecordingWithListens>,
    minimum_listens: u64,
) -> impl Stream<Item = RecordingWithListens> {
    listens.filter(move |r| ready(r.len() as u64 >= minimum_listens))
}
