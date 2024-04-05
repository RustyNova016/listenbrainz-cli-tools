use std::{
    fs::File,
    hash::Hash,
    path::PathBuf,
    sync::Arc,
};

use chashmap::CHashMap;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};

use crate::utils::println_cli;

use super::CACHE_LOCATION;

#[derive(Debug, Clone)]
pub struct DiskCache<K, V> {
    data: CHashMap<Arc<K>, Arc<V>>,
    filename: String,
}

impl<K, V> DiskCache<K, V>
where
    K: Serialize + DeserializeOwned + PartialEq + Hash,
    V: Serialize + DeserializeOwned,
{
    pub fn new(filename: String) -> Self {
        Self {
            data: CHashMap::with_capacity(1000),
            filename,
        }
    }

    pub fn get(&self, key: &K) -> Option<Arc<V>> where {
        self.data.get(key).map(|inner| inner.clone())
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn insert(&self, key: Arc<K>, value: Arc<V>) -> Option<Arc<V>> {
        self.data.insert(key, value)
    }

    pub fn get_file_path(&self) -> PathBuf {
        let mut path = CACHE_LOCATION.clone();
        path.push(self.filename.clone());
        path
    }

    pub fn to_vec(&self) -> Vec<(Arc<K>, Arc<V>)> {
        self.data
            .clone() // This makes a cheap clone as all the entries are references
            .into_iter()
            .collect_vec()
    }

    pub fn save_cache(&self) -> color_eyre::Result<()> {
        let file = File::create(self.get_file_path())?;

        let json_vec = self.to_vec();
        serde_json::to_writer(file, &json_vec)?;

        Ok(())
    }

    fn load_data(&self) -> color_eyre::Result<()> {
        let cache_file = File::open(self.get_file_path())?;
        let cache_data: Vec<(K, V)> = serde_json::from_reader(cache_file)?;

        for (key, value) in cache_data {
            self.data.insert(key.into(), value.into());
        }

        Ok(())
    }

    pub fn load_or_new(filename: String) -> Self {
        let new = Self::new(filename);

        let Ok(_) = new.load_data() else {
            println_cli(
                format!(
                    "Failed to load cache `{}`. A new one will get created",
                    new.get_filename()
                )
                .red(),
            );

            return new;
        };

        println_cli(format!(
            "Loaded {} artists from cache `{}`",
            new.len(),
            new.get_filename()
        ));

        new
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
