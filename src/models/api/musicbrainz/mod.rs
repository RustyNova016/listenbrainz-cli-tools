use colored::Colorize;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::models::cache::artist_cache::ArtistCache;
use crate::models::cache::recording_cache::RecordingCache;
use crate::models::cache::DiskCache;
use crate::models::data::recording::Recording;
use crate::utils::println_cli;

pub mod artist;
pub mod artist_credit;
pub mod recording;

pub(crate) static mut MUSICBRAINZ_API: Lazy<Mutex<MusicBrainzAPI>> =
    Lazy::new(|| Mutex::new(MusicBrainzAPI::new()));

#[derive(Debug)]
pub struct MusicBrainzAPI {
    recording_cache: RecordingCache,
    artist_cache: ArtistCache,
    fetch_count: u32,
}

impl MusicBrainzAPI {
    pub fn new() -> Self {
        Self {
            fetch_count: 0,
            recording_cache: RecordingCache::load_from_disk_or_new(),
            artist_cache: ArtistCache::load_from_disk_or_new(),
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
