use std::sync::Arc;

use chrono::{DateTime, Utc};
use listenbrainz::raw::response::UserListensListen;
use crate::models::data::musicbrainz::MBIDType;
use crate::utils::traits::VecWrapper;

use super::UserListen;

/// Collection of listens
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserListenCollection {
    data: Vec<Arc<UserListen>>,
}

impl UserListenCollection {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_mapped_listens(&self) -> UserListenCollection {
        self.data
            .iter()
            .filter(|element| element.is_mapped())
            .cloned()
            .collect()
    }

    pub fn get_unmapped_listens(&self) -> UserListenCollection {
        self.data
            .iter()
            .filter(|element| !element.is_mapped())
            .cloned()
            .collect()
    }

    /// Remove all the listens in between two dates.
    pub fn remove_period(&mut self, start: DateTime<Utc>, end: DateTime<Utc>, inclusive: bool) {
        self.data.retain(|listen| {
            if inclusive {
                listen.listened_at < start || end < listen.listened_at
            } else {
                listen.listened_at <= start || end <= listen.listened_at
            }
        })
    }

    pub fn get_latest_listen(&self) -> Option<Arc<UserListen>> {
        self.data
            .iter()
            .max_by_key(|listen| listen.listened_at)
            .cloned()
    }

    pub fn get_listens_for_mbid<'a>(
        &'a self,
        mbid: &'a str,
        mbid_type: MBIDType,
    ) -> Vec<Arc<UserListen>> {
        match mbid_type {
            MBIDType::Recording => self.get_listen_with_recording(mbid),
            MBIDType::Artist => todo!(),
        }
    }

    fn get_listen_with_recording<'a>(&'a self, recording_mbid: &'a str) -> Vec<Arc<UserListen>> {
        self.get_mapped_listens()
            .into_iter()
            .filter(|listen| {
                listen
                    .mapping_data
                    .as_ref()
                    .is_some_and(|mapping| mapping.recording_mbid == recording_mbid)
            })
            .collect()
    }

    pub fn push<T>(&mut self, item: T)
    where
        T: Into<Arc<UserListen>>,
    {
        self.data.push(item.into())
    }

    pub fn get(&self, index: usize) -> Option<Arc<UserListen>> {
        self.data.get(index).cloned()
    }
}

impl VecWrapper<Arc<UserListen>> for UserListenCollection {
    fn get_vec(&self) -> &Vec<Arc<UserListen>> {
        &self.data
    }
}

impl TryFrom<Vec<UserListensListen>> for UserListenCollection {
    type Error = &'static str;

    fn try_from(value: Vec<UserListensListen>) -> Result<Self, Self::Error> {
        let mut data: Vec<Arc<UserListen>> = Vec::new();

        for listen in value.into_iter() {
            data.push(Arc::new(listen.try_into()?))
        }

        Ok(Self { data })
    }
}

impl FromIterator<UserListen> for UserListenCollection {
    fn from_iter<T: IntoIterator<Item = UserListen>>(iter: T) -> Self {
        let mut coll = Self::new();

        for ele in iter {
            coll.push(ele)
        }

        coll
    }
}

impl FromIterator<Arc<UserListen>> for UserListenCollection {
    fn from_iter<T: IntoIterator<Item = Arc<UserListen>>>(iter: T) -> Self {
        let mut coll = Self::new();

        for ele in iter {
            coll.data.push(ele)
        }

        coll
    }
}

impl IntoIterator for UserListenCollection {
    type Item = Arc<UserListen>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
