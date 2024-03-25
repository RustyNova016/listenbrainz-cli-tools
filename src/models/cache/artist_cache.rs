use moka::sync::Cache;

use std::fs::File;

use crate::{models::data::recording::Artist, utils::println_cli};

use super::{CacheWrapper, DiskCache};

#[derive(Debug)]
pub struct ArtistCache {
    cache: Cache<String, Artist>,
}

impl CacheWrapper<String, Artist> for ArtistCache {
    fn get_cache(&self) -> &Cache<String, Artist> {
        &self.cache
    }

    fn get_cache_mut(&mut self) -> &mut Cache<String, Artist> {
        &mut self.cache
    }
}

impl<'de> DiskCache<'de, String, Artist> for ArtistCache {
    fn save_cache(&self) -> color_eyre::Result<()> {
        let file = File::create(Self::get_file_path())?;

        serde_json::to_writer(file, &self.to_json_vec())?;

        Ok(())
    }

    fn load_cache(&mut self) -> color_eyre::Result<()> {
        let file = File::open(Self::get_file_path())?;
        let cache_vec: Vec<(String, Artist)> = serde_json::from_reader(file)?;

        for (key, value) in cache_vec {
            self.cache.insert(key, value);
        }

        println_cli(&format!(
            "Loaded {} artists from cache",
            self.cache.entry_count()
        ));

        Ok(())
    }

    fn get_filename() -> &'static str {
        todo!()
    }

    fn get_file_path() -> std::path::PathBuf {
        "C:\\test\\artists.json".into()
    }

    fn new() -> Self {
        Self {
            cache: Cache::builder().max_capacity(100000).build(),
        }
    }

    fn load_from_disk_or_new() -> Self {
        let mut cache = Self::new();
        let res = cache.load_cache();
        if res.is_err() {
            println_cli("Couldn't load the artist cache file. Creating a new one");
            Self::new()
        } else {
            cache
        }
    }
}
