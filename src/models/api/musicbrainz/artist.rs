use color_eyre::owo_colors::OwoColorize;
use musicbrainz_rs::{entity, Fetch};

use crate::models::data::recording::Artist;

use super::MusicBrainzAPI;

impl MusicBrainzAPI {
    pub fn get_artist(&mut self, mbid: String) -> Artist {
        let cached = self.artist_cache.get(&mbid);

        if let Some(cach) = cached {
            cach
        } else {
            self.fetch_artist(mbid)
        }
    }

    pub fn fetch_artist(&mut self, mbid: String) -> Artist {
        println!(
            "{} Getting data for artist MBID: {}",
            "[MusicBrainz]".bright_magenta(),
            &mbid
        );
        let response = entity::artist::Artist::fetch().id(&mbid).execute();

        if let Ok(msartist) = response {
            let artist = Artist::from(msartist);
            self.insert_artist(artist.id.clone(), artist.clone());

            // In cases where the MBID asked isn't the same as the one we received, we also cache it.
            // This may due to a merge, and we have a old MBID that is redirecting to a new one.
            self.insert_artist(mbid.to_string(), artist);

            self.fetch_count += 1;
            self.autosave();

            self.artist_cache
                .get(&mbid)
                .expect("Failed to get record from cache after insertion")
        } else {
            panic!("Failed to fetch recording from MusicBrainz")
        }
    }

    pub fn insert_artist(&self, mbid: String, value: Artist) {
        self.artist_cache.insert(mbid, value)
    }
}
