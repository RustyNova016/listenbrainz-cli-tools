use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;

use super::StatSorter;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct ArtistStatsSorter {
    listens: HashMap<String, ListenCollection>,
}

impl ArtistStatsSorter {
    pub fn new() -> Self {
        Self {
            listens: HashMap::new(),
        }
    }
}

impl StatSorter for ArtistStatsSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, ListenCollection> {
        &mut self.listens
    }

    fn push(&mut self, value: Arc<Listen>) -> color_eyre::Result<()> {
        let Some(recording_data) = value.get_recording_data()? else {
            return Ok(());
        };

        let artist_credits = recording_data.get_or_fetch_artist_credits()?;
        for artist_id in artist_credits.get_artist_ids() {
            self.get_mut(&artist_id).push(value.clone());
        }

        Ok(())
    }

    fn into_vec(self) -> Vec<(String, ListenCollection)> {
        self.listens.into_iter().collect_vec()
    }
}
