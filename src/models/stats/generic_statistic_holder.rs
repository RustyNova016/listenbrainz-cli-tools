use std::sync::{Arc, Mutex};

use crate::models::data::listenbrainz::listen::{collection::ListenCollection, Listen};

use super::StatisticHolder;

pub struct GenericStatisticHolder<K> {
    id: K,
    listens: Mutex<ListenCollection>
}

impl<K> StatisticHolder<String> for GenericStatisticHolder<K> {
    fn insert_listen(&self, listen: Arc<Listen>) -> impl std::future::Future<Output = color_eyre::eyre::Result<()>> {
        async {
            let mut listens = self.listens.lock().unwrap();

            listens.push(listen);
            drop(listens);
            Ok(())
        }
    }

    fn count(&self) -> usize {
        self.listens.lock().unwrap().len()
    }

    fn create(id: K) -> Self {
        Self {
            id,
            listens: Mutex::new(ListenCollection::new())
        }
    }
}