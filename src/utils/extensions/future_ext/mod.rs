
use std::sync::Arc;

use color_eyre::eyre::Report;
use extend::ext;
use futures::future;
use futures::Stream;
use futures::StreamExt;
use futures::TryStream;

use crate::models::data::listenbrainz::listen::listen_mapped::NaiveMappedListen;
use crate::models::data::listenbrainz::listen::mapped_primary::MappedListen;

#[ext]
pub impl<S, I> S where S: Stream<Item = I> + StreamExt {
    /// Put all the item into a future then buffer them
    fn buffer_unordered_non_future(self, n: usize) -> impl Stream<Item = I> {
        self.map(|item| future::ready(item)).buffer_unordered(n)
    }

        /// Put all the item into a future then buffer them
        fn buffered_non_future(self, n: usize) -> impl Stream<Item = I> {
            self.map(|item| future::ready(item)).buffered(n)
        }
}
