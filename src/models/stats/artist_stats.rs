use std::rc::Rc;

use musicbrainz_rs::{entity::artist::Artist, Fetch};

use crate::models::data::listens::UserListen;

use super::stat_struct::StatStruct;

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
