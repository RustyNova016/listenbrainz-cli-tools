use color_eyre::eyre::Context;
use listenbrainz::raw::Client;

use crate::models::cache::{listen_cache::ListenCache, DiskCache};

pub mod user_listens;

#[derive(Debug)]
pub struct ListenBrainzAPI {
    listen_cache: ListenCache,
    api_client: Client,
}

impl Default for ListenBrainzAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl ListenBrainzAPI {
    pub fn new() -> Self {
        Self {
            listen_cache: ListenCache::load_from_disk_or_new(),
            api_client: Client::new(),
        }
    }

    pub fn save_cache(&self) -> color_eyre::Result<()> {
        self.listen_cache
            .save_cache()
            .context("Couldn't save Listens cache")
    }
}
