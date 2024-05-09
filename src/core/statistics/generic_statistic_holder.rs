use crate::core::statistics::statistic_holder::StatisticHolder;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use derive_new::new;
use std::sync::{Arc, Mutex};

#[derive(Debug, new)]
pub struct GenericStatisticHolder<K> {
    _id: K,
    #[new(default)]
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
