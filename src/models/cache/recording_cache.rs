use crate::models::cache::CacheWrapper;
use crate::models::data::recording::Recording;
use crate::utils::println_cli;
use moka::sync::Cache;
use std::fs::File;

#[derive(Debug)]
pub struct RecordingCache {
    cache: Cache<String, Recording>,
}

impl Default for RecordingCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordingCache {
    pub fn new() -> Self {
        RecordingCache {
            cache: Cache::builder().max_capacity(100000).build(),
        }
    }

    pub fn save_cache(&self) -> color_eyre::Result<()> {
        let cache_vec: Vec<(String, Recording)> = self
            .cache
            .iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect();

        let file = File::create("C:\\test\\recordings.json")?;

        serde_json::to_writer(file, &cache_vec)?;

        Ok(())
    }

    pub fn load_from_disk(&self) -> color_eyre::Result<()> {
        let file = File::open("C:\\test\\recordings.json")?;
        let cache_vec: Vec<(String, Recording)> = serde_json::from_reader(file)?;

        for (key, value) in cache_vec {
            self.cache.insert(key, value);
        }

        println_cli(&format!(
            "Loaded {} recordings from cache",
            self.cache.entry_count()
        ));
        Ok(())
    }

    pub fn load_from_disk_or_new() -> Self {
        let cache = Self::new();
        let res = cache.load_from_disk();

        if res.is_err() {
            println_cli("Couldn't load the recording cache file. Creating a new one");
            Self::new()
        } else {
            cache
        }
    }

    pub fn insert(&self, key: String, value: Recording) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Recording> {
        self.cache.get(key)
    }

    pub fn cache(&self) -> &Cache<String, Recording> {
        &self.cache
    }
}
