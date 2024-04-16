use std::sync::{Arc, Mutex};

use crate::models::data::listenbrainz::listen::{collection::ListenCollection, Listen};

use super::StatisticHolder;

pub struct GenericStatisticHolder<K> {
    _id: K,
    listens: Mutex<ListenCollection>,
}

impl<K> StatisticHolder<K> for GenericStatisticHolder<K> {
    async fn insert_listen(&self, listen: Arc<Listen>) -> color_eyre::eyre::Result<()> {
        let mut listens = self.listens.lock().unwrap();

        listens.push(listen);
        drop(listens);
        Ok(())
    }

    fn count(&self) -> usize {
        self.listens.lock().unwrap().len()
    }

    fn create(id: K) -> Self {
        Self {
            _id: id,
            listens: Mutex::new(ListenCollection::new()),
        }
    }
}
