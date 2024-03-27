use std::{collections::HashMap, rc::Rc, sync::Arc};

use crate::models::data::listens::collection::UserListenCollection;
use itertools::Itertools;
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

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct ArtistStatsSorter {
    listens: HashMap<String, UserListenCollection>,
}

impl ArtistStatsSorter {
    pub fn new() -> Self {
        Self {
            listens: HashMap::new(),
        }
    }
}

impl StatSorter for ArtistStatsSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, UserListenCollection> {
        &mut self.listens
    }

    fn push(&mut self, value: Arc<UserListen>) {
        let Some(recording_data) = value.get_recording_data() else {
            return;
        };

        for artist_credited in recording_data.artist_credit.unwrap_or(Vec::new()) {
            self.get_mut(&artist_credited.artist.id).push(value.clone());
        }
    }

    fn into_vec(self) -> Vec<(String, UserListenCollection)> {
        self.listens.into_iter().collect_vec()
    }
}
