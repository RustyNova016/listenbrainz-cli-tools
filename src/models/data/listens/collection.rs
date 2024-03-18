use listenbrainz::raw::response::UserListensListen;

use crate::models::musicbrainz::MBIDType;

use super::UserListen;

/// Collection of listens
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserListenCollection {
    data: Vec<UserListen>,
}

impl UserListenCollection {
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    pub fn get_mapped_listens(&self) -> impl Iterator<Item = &UserListen> {
        self.data.iter().filter(|element| element.is_mapped())
    }

    pub fn get_listens_for_mbid<'a>(
        &'a self,
        mbid: &'a str,
        mbid_type: MBIDType,
    ) -> Vec<&UserListen> {
        match mbid_type {
            MBIDType::Recording => self.get_listen_with_recording(mbid),
        }
    }

    fn get_listen_with_recording<'a, 'b>(&'a self, recording_mbid: &'a str) -> Vec<&UserListen> {
        self.get_mapped_listens()
            .filter(|listen| {
                listen
                    .mapping_data
                    .as_ref()
                    .is_some_and(|mapping| mapping.recording_mbid == recording_mbid)
            })
            .collect()
    }

    pub fn push(&mut self, item: UserListen) {
        self.data.push(item)
    }
}

impl TryFrom<Vec<UserListensListen>> for UserListenCollection {
    type Error = &'static str;

    fn try_from(value: Vec<UserListensListen>) -> Result<Self, Self::Error> {
        let mut data: Vec<UserListen> = Vec::new();

        for listen in value.into_iter() {
            data.push(listen.try_into()?)
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