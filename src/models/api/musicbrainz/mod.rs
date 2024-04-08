use once_cell::sync::Lazy;

use crate::models::cache::artist_cache::ArtistCache;
use crate::models::cache::recording_cache::RecordingCache;
use crate::models::cache::DiskCache;

#[derive(Debug)]
pub struct MusicBrainzAPI {
    recording_cache: Lazy<RecordingCache>,
    artist_cache: Lazy<ArtistCache>,
}

impl Default for MusicBrainzAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicBrainzAPI {
    pub fn new() -> Self {
        Self {
            recording_cache: Lazy::new(RecordingCache::new),
            artist_cache: Lazy::new(ArtistCache::new),
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
