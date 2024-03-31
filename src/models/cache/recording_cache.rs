use super::{CacheWrapper, DiskCache};
use crate::models::data::recording::Recording;
use crate::utils::println_cli;
use color_eyre::owo_colors::OwoColorize;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::ops::Deref;
use std::path::Path;

pub(crate) static RECORDING_CACHE: Lazy<Cache<String, Recording>> =
    Lazy::new(create_recording_cache);

fn create_recording_cache() -> Cache<String, Recording> {
    let Ok(cache) = RecordingCache::try_load_new_cache() else {
        println_cli("Failed to load recordings cache. A new one will get created".red());

        return RecordingCache::get_base_cache();
    };

    cache.run_pending_tasks();
    println_cli(format!(
        "Loaded {} recordings from cache",
        cache.entry_count()
    ));

    cache
}

#[derive(Debug, Clone)]
pub struct RecordingCache {
    cache: Cache<String, Recording>,
}

impl Default for RecordingCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for RecordingCache {
    type Target = Cache<String, Recording>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl CacheWrapper<String, Recording> for RecordingCache {}

impl DiskCache<String, Recording> for RecordingCache {
    fn new() -> Self {
        RecordingCache {
            cache: RECORDING_CACHE.clone(),
        }
    }

    fn get_filename() -> &'static Path {
        Path::new("recordings.json")
    }

    fn get_static_cache() -> &'static Lazy<Cache<String, Recording>> {
        &RECORDING_CACHE
    }
}
