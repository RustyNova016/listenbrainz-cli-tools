use crate::models::data::recording::Recording;
use crate::utils::println_cli;
use moka::sync::Cache;
use std::fs::File;
use std::ops::AddAssign;
use std::sync::Mutex;
use crate::models::cache::CacheWrapper;

#[derive(Debug)]
pub struct RecordingCache {
    cache: Cache<String, Recording>,
}

impl RecordingCache {
    pub fn new() -> Self {
        RecordingCache {
            cache: Cache::builder().max_capacity(100000).build(),
        }
    }

    pub fn save_to_disk(&self) -> color_eyre::Result<()> {
        let cache_vec: Vec<(String, Recording)> = self
            .cache
            .iter()
            .map(|(key, value)| (key.to_string(), value.into()))
            .collect();

        let file = File::create("/home/rustynova/.cache/listenbrainz_cli_tools/recordings.json")?;

        serde_json::to_writer(file, &cache_vec)?;

        Ok(())
    }

    pub fn load_from_disk(&self) -> color_eyre::Result<()> {
        let file = File::open("/home/rustynova/.cache/listenbrainz_cli_tools/recordings.json")?;
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
        let mut cache = Self::new();
        let res = cache.load_from_disk();
        if res.is_err() {
            println_cli("Couldn't load the cache file. Creating a new one");
            Self::new()
        } else {
            cache
        }
    }

    pub fn insert(&mut self, key: String, value: Recording) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Recording> {
        self.cache.get(key)
    }

    pub fn cache(&self) -> &Cache<String, Recording> {
        &self.cache
    }
}

impl CacheWrapper<String, Recording> {
    
}