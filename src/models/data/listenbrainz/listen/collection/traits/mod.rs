use std::sync::Arc;

use crate::models::data::listenbrainz::listen::Listen;

pub mod listen_iterator;

pub trait CollectionOfListens {
    fn iter_listens(&self) -> impl Iterator<Item = &Arc<Listen>>;

    /// Return the oldest listen of the collection
    fn find_oldest_listen(&self) -> Option<&Arc<Listen>> {
        self.iter_listens()
            .min_by_key(|listen| *listen.listened_at())
    }

    /// Return the latest listen of the collection
    fn find_latest_listen(&self) -> Option<&Arc<Listen>> {
        self.iter_listens()
            .max_by_key(|listen| *listen.listened_at())
    }
}
