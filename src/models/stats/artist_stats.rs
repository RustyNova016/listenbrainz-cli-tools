use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::core::statistics::statistic_holder::StatisticHolder;
use crate::core::statistics::statistic_sorter_trait::StatisticSorter;
use chashmap::CHashMap;
use itertools::Itertools;
use std::sync::{Arc, Mutex};

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;

#[derive(Debug, Default)]
pub struct ArtistStatisticHolder {
    listens: Mutex<ListenCollection>,
}

impl StatisticHolder<String> for ArtistStatisticHolder {
    async fn insert_listen(&self, listen: Arc<Listen>) -> color_eyre::eyre::Result<()> {
        let mut listens = self.listens.lock().unwrap();

        listens.push(listen);
        drop(listens);
        Ok(())
    }

    fn count(&self) -> usize {
        self.listens.lock().unwrap().len()
    }

    fn create(_id: String) -> Self {
        Self {
            listens: Mutex::new(ListenCollection::new()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ArtistStatisticSorter {
    data: CHashMap<String, Arc<ArtistStatisticHolder>>,
}

impl ArtistStatisticSorter {
    pub fn new() -> Self {
        Self {
            data: CHashMap::new(),
        }
    }
}

impl StatisticSorter<String, ArtistStatisticHolder> for ArtistStatisticSorter {
    fn insert_listen(
        &self,
        listen: Arc<Listen>,
    ) -> impl std::future::Future<Output = color_eyre::eyre::Result<()>> {
        let listen = listen.clone();
        async move {
            let Some(recording_data) = listen.get_recording_data().await? else {
                return Ok(());
            };

            let artist_credits = recording_data.get_or_fetch_artist_credits().await?;
            for artist_id in artist_credits.get_artist_ids() {
                self.get(&artist_id).insert_listen(listen.clone()).await?;
            }

            Ok(())
        }
    }

    fn get(&self, key: &String) -> Arc<ArtistStatisticHolder> {
        let collection = self.data.get(key);

        if let Some(collection) = collection {
            return collection.clone();
        }

        self.data.insert(
            key.to_string(),
            Arc::new(ArtistStatisticHolder::create(key.to_string())),
        );
        self.data
            .get(key)
            .map(|collection| collection.clone())
            .expect("Couldn't retrieve inserted collection")
    }

    fn into_vec(self) -> Vec<(String, Arc<ArtistStatisticHolder>)> {
        self.data.into_iter().collect_vec()
    }
}
