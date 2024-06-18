use std::sync::Arc;

use color_eyre::eyre::Report;
use extend::ext;
use futures::Stream;
use futures::StreamExt;
use futures::TryStream;
use futures::TryStreamExt;

use crate::models::data::listenbrainz::listen::listen_mapped::NaiveMappedListen;
use crate::models::data::listenbrainz::listen::mapped_primary::MappedListen;

#[ext]
pub impl<S> S
where
    S: Stream<Item = NaiveMappedListen> + StreamExt,
{
    fn into_primary(self) -> impl Stream<Item = color_eyre::Result<MappedListen>> + TryStream + TryStreamExt{
        self.map(|listen| async move { listen.into_primary_mapping().await })
            .buffer_unordered(1)
    }
}
