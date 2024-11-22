use core::ops::Deref;

use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use serde::Deserialize;
use serde::Serialize;

pub mod traits;

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct ListenCollection {
    pub data: Vec<Listen>,
}

impl ListenCollection {
    pub fn new(data: Vec<Listen>) -> Self {
        Self { data }
    }

    /// Returns the latest listen in the collection.
    pub fn get_latest_listen(&self) -> Option<&Listen> {
        self.data.iter().max_by_key(|listen| listen.listened_at)
    }

    /// Returns the oldest listen in the collection.
    pub fn get_oldest_listen(&self) -> Option<&Listen> {
        self.data.iter().min_by_key(|listen| listen.listened_at)
    }

    pub fn push(&mut self, listen: Listen) {
        self.data.push(listen);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the first element of the collection
    ///
    /// To return the oldest listen use `ListenCollection::get_oldest_listen`
    pub fn first(&self) -> Option<&Listen> {
        self.data.first()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Listen> {
        self.data.iter()
    }
}

impl From<Vec<Listen>> for ListenCollection {
    fn from(value: Vec<Listen>) -> Self {
        Self { data: value }
    }
}

impl Deref for ListenCollection {
    type Target = Vec<Listen>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
