use std::rc::Rc;

use musicbrainz_rs::{entity::artist::Artist, Fetch};

use crate::models::data::listens::UserListen;

pub struct ArtistStats {
    mbid: String,

    listens: Vec<Rc<UserListen>>,
}

impl ArtistStats {
    pub fn get_name(&self) -> String {
        Artist::fetch().id(&self.mbid).execute().unwrap().name //TODO: Remove ugly unwrap
    }

    pub fn push(&mut self, item: Rc<UserListen>) {
        self.listens.push(item)
    }

    pub fn get_mbid(&self) -> &str {
        &self.mbid
    }
}