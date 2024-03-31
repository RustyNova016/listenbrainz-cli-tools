use once_cell::sync::Lazy;

use crate::models::cache::artist_cache::ArtistCache;
use crate::models::cache::recording_cache::RecordingCache;
use crate::models::cache::DiskCache;
use crate::utils::println_cli;

pub mod artist;
pub mod artist_credit;
pub mod recording;

#[derive(Debug)]
pub struct MusicBrainzAPI {
    recording_cache: Lazy<RecordingCache>,
    artist_cache: Lazy<ArtistCache>,
    fetch_count: u32,
}

impl Default for MusicBrainzAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicBrainzAPI {
    pub fn new() -> Self {
        Self {
            fetch_count: 0,
            recording_cache: Lazy::new(RecordingCache::new),
            artist_cache: Lazy::new(ArtistCache::new),
        }
    }

    fn autosave(&mut self) {
        if self.fetch_count % 100 == 0 {
            println_cli("Autosaving the cache file...");
            self.save_cache()
        }
    }

    pub fn save_cache(&mut self) {
        self.recording_cache
            .save_cache()
            .expect("IO Error: Failed to save recording cache file");

        self.artist_cache
            .save_cache()
            .expect("IO Error: Failed to save artists cache file");
    }
}
