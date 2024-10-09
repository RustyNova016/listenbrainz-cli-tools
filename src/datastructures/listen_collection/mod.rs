use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

pub mod group_by;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
        self.data.push(listen)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl From<Vec<Listen>> for ListenCollection {
    fn from(value: Vec<Listen>) -> Self {
        Self { data: value }
    }
}

