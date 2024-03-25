use std::{cmp::Reverse, collections::HashMap, rc::Rc};

use crate::models::api::musicbrainz::MusicBrainzAPI;
use musicbrainz_rs::{entity::artist::Artist, Fetch};

use crate::models::data::listens::UserListen;

use super::{stat_struct::StatStruct, StatSorter};

pub struct ArtistStats {
    mbid: String,

    listens: Vec<Rc<UserListen>>,
}

impl ArtistStats {
    pub fn get_name(&self) -> String {
        Artist::fetch().id(&self.mbid).execute().unwrap().name //TODO: Remove ugly unwrap
    }
}

impl StatStruct for ArtistStats {
    fn get_mbid(&self) -> &str {
        &self.mbid
    }

    fn new(mbid: String) -> Self {
        Self {
            listens: Vec::new(),
            mbid,
        }
    }

    fn push(&mut self, item: Rc<UserListen>) {
        if item
            .get_mapping_data()
            .as_ref()
            .is_some_and(|mapdata| mapdata.get_artists_mbids().contains(&self.mbid))
        {
            self.listens.push(item)
        }
    }
}

pub struct ArtistStatsSorter {
    listens: HashMap<String, Vec<Rc<UserListen>>>,
}

impl ArtistStatsSorter {
    pub fn new() -> Self {
        Self {
            listens: HashMap::new(),
        }
    }
}

impl StatSorter for ArtistStatsSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, Vec<Rc<UserListen>>> {
        &mut self.listens
    }

    fn push(&mut self, value: Rc<UserListen>, mb_client: &mut MusicBrainzAPI) {
        let Some(recording_data) = value.get_recording_data(mb_client) else {
            return;
        };

        for artist_credited in recording_data.artist_credit.unwrap_or(Vec::new()) {
            self.get_mut(&artist_credited.artist.id).push(value.clone());
        }
    }

    fn into_sorted(self) -> Vec<Vec<Rc<UserListen>>> {
        let mut out = Vec::new();
        out.extend(self.listens.into_values());
        out.sort_unstable_by_key(|item| Reverse(item.len()));
        out
    }
}
