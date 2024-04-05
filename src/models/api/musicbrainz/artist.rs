use std::sync::Arc;

use color_eyre::owo_colors::OwoColorize;
use musicbrainz_rs::{entity, Fetch};

use crate::models::{cache::global_cache::GlobalCache, data::recording::Artist};
use crate::models::cache::cached_trait::Cached;

use super::MusicBrainzAPI;

impl Cached<String, Artist> for Artist {
    fn get_cached(key: &String) -> Option<Arc<Artist>> {
        GlobalCache::new().get_artist(&key)
    }

    fn fetch(key: &String) -> color_eyre::Result<Arc<Artist>> {
        println!(
            "{} Getting data for artist MBID: {}",
            "[MusicBrainz]".bright_magenta(),
            &key
        );
        let response = entity::artist::Artist::fetch().id(&key).with_recordings().execute();

        if let Ok(msartist) = response {
            let artist = Arc::new(Artist::from(msartist));
            let cache = GlobalCache::new();

            cache.insert_artist(artist.id.clone().into(), artist.clone());

            // In cases where the MBID asked isn't the same as the one we received, we also cache it.
            // This may due to a merge, and we have a old MBID that is redirecting to a new one.
            cache.insert_artist(key.clone().into(), artist.clone());

            Ok(artist)
        } else {
            Err("Failed to fetch recording from MusicBrainz")?
        }
    }
}

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
