use colored::Colorize;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::models::cache::recording_cache::RecordingCache;
use crate::models::data::recording::Recording;
use crate::utils::println_cli;

pub(crate) static mut MUSICBRAINZ_API: Lazy<Mutex<MusicBrainzAPI>> =
    Lazy::new(|| Mutex::new(MusicBrainzAPI::new()));

#[derive(Debug)]
pub struct MusicBrainzAPI {
    recording_cache: RecordingCache,
    fetch_count: u32,
}

impl MusicBrainzAPI {
    pub fn new() -> Self {
        Self {
            fetch_count: 0,
            recording_cache: RecordingCache::load_from_disk_or_new(),
        }
    }

    pub fn get_recording_data(&mut self, mbid: &str) -> Recording {
        //self.recording_cache.get(&mbid).unwrap_or(self.fetch_recording(mbid))

        let cached = self.recording_cache.get(&mbid);

        if let Some(cach) = cached {
            println!("Cache hit for: {}", mbid);
            cach
        } else {
            self.fetch_recording(mbid)
        }
    }

    fn fetch_recording(&mut self, mbid: &str) -> Recording {
        println!(
            "{} Getting data for recording MBID: {}",
            "[MusicBrainz]".bright_magenta(),
            &mbid
        );
        let response = Recording::fetch().id(&mbid).with_artists().execute();

        if let Ok(msrecord) = response {
            let record = Recording::from(msrecord);
            self.recording_cache
                .insert(record.id.clone(), record.clone());

            // In cases where the MBID asked isn't the same as the one we received, we also cache it.
            // This may due to a merge, and we have a old MBID that is redirecting to a new one.
            self.recording_cache.insert(mbid.to_string(), record);

            self.fetch_count += 1;
            self.autosave();

            self.recording_cache
                .get(&mbid)
                .expect("Failed to get record from cache after insertion")
        } else {
            panic!("Failed to fetch recording from MusicBrainz")
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
            .save_to_disk()
            .expect("IO Error: Failed to save cache file");
    }
}
