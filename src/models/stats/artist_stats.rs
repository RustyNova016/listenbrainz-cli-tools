use chashmap::CHashMap;
use clap::builder::Str;
use itertools::Itertools;
use tokio::sync::futures;
use std::{collections::HashMap, sync::Mutex};
use std::sync::Arc;

use crate::models::data::listenbrainz::listen::collection::{self, ListenCollection};
use crate::models::data::listenbrainz::listen::Listen;

use super::{StatSorter, StatisticHolder, StatisticSorter};

pub struct ArtistStatisticHolder {
    id: String,
    listens: Mutex<ListenCollection>
}

impl StatisticHolder<String> for ArtistStatisticHolder {
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

    fn create(id: String) -> Self {
        Self {
            id,
            listens: Mutex::new(ListenCollection::new())
        }
    }
}

pub struct ArtistStatisticSorter {
    data: CHashMap<String, Arc<ArtistStatisticHolder>>
}

impl StatisticSorter<String, ArtistStatisticHolder> for ArtistStatisticSorter {
    fn insert_listen(&self, listen: Arc<Listen>) -> impl std::future::Future<Output = color_eyre::eyre::Result<()>> {
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

        self.data.insert(key.to_string(), Arc::new(ArtistStatisticHolder::create(key.to_string())));
        self.data.get(key).map(|collection| collection.clone()).expect("Couldn't retrieve inserted collection")
    }
}