use super::{CacheWrapper, DiskCache};
use crate::{models::data::recording::Artist, utils::println_cli};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::ops::Deref;
use std::path::Path;

use color_eyre::owo_colors::OwoColorize;

pub(crate) static ARTIST_CACHE: Lazy<Cache<String, Artist>> = Lazy::new(create_artist_cache);

fn create_artist_cache() -> Cache<String, Artist> {
    let Ok(cache) = ArtistCache::try_load_new_cache() else {
        println_cli("Failed to load artists cache. A new one will get created".red());

        return ArtistCache::get_base_cache();
    };

    cache.run_pending_tasks();
    println_cli(format!("Loaded {} artists from cache", cache.entry_count()));

    cache
}

#[derive(Debug, Clone)]
pub struct ArtistCache {
    cache: Cache<String, Artist>,
}

impl Deref for ArtistCache {
    type Target = Cache<String, Artist>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl CacheWrapper<String, Artist> for ArtistCache {}

impl DiskCache<String, Artist> for ArtistCache {
    fn get_filename() -> &'static Path {
        Path::new("artists.json")
    }

    /// Mutiple caches can be safely started as they are only references to the same [`Cache`](`moka::sync::cache::Cache`) object
    fn new() -> Self {
        Self {
            cache: ARTIST_CACHE.clone(),
        }
    }

    fn get_static_cache() -> &'static Lazy<Cache<String, Artist>> {
        &ARTIST_CACHE
    }
}
