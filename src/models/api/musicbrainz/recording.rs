use color_eyre::owo_colors::OwoColorize;

use crate::models::data::recording::Recording;

use super::MusicBrainzAPI;

impl MusicBrainzAPI {
    /// Return the data about a recording from musicbrainz. It will try to get it from the cache first
    pub fn get_recording_data(&mut self, mbid: &str) -> Recording {
        //self.recording_cache.get(&mbid).unwrap_or(self.fetch_recording(mbid))

        let cached = self.recording_cache.get(mbid);

        if let Some(cach) = cached {
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
        let response = Recording::fetch().id(mbid).with_artists().execute();

        if let Ok(msrecord) = response {
            let record = Recording::from(msrecord);
            self.insert_recording(record.id.clone(), record.clone());

            // In cases where the MBID asked isn't the same as the one we received, we also cache it.
            // This may due to a merge, and we have a old MBID that is redirecting to a new one.
            if mbid != record.id {
                self.insert_recording(mbid.to_string(), record);
            }

            self.fetch_count += 1;
            self.autosave();

            self.recording_cache
                .get(mbid)
                .expect("Failed to get record from cache after insertion")
        } else {
            panic!("Failed to fetch recording from MusicBrainz")
        }
    }

    fn insert_recording(&self, mbid: String, value: Recording) {
        self.recording_cache.insert(mbid.clone(), value.clone());

        if let Some(artist_credit_vec) = value.artist_credit {
            self.insert_artist_credits(artist_credit_vec);
        }
    }
}
