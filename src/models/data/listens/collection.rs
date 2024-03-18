use std::rc::Rc;

use listenbrainz::raw::response::UserListensListen;

use crate::models::musicbrainz::MBIDType;

use super::UserListen;

/// Collection of listens
pub struct UserListenCollection {
    data: Vec<Rc<UserListen>>,
}

impl UserListenCollection {
    pub fn get_mapped_listens(&self) -> Vec<Rc<UserListen>> {
        self.data.iter().filter(|element| element.is_mapped()).cloned().collect()
    }

    pub fn get_listens_for_mbid(&self, mbid: &str, mbid_type: MBIDType) -> Vec<Rc<UserListen>>  {
        match mbid_type {
            MBIDType::Recording => {self.get_listen_with_recording(mbid)}
        }
    }

    fn get_listen_with_recording(&self, recording_mbid: &str) -> Vec<Rc<UserListen>> {
        self.get_mapped_listens().into_iter().filter(|listen| listen.mapping_data.as_ref().is_some_and(|mapping| mapping.recording_mbid == recording_mbid)).collect()
    }
}

impl TryFrom<Vec<UserListensListen>> for UserListenCollection {
    type Error = &'static str;

    fn try_from(value: Vec<UserListensListen>) -> Result<Self, Self::Error> {
        let mut data: Vec<Rc<UserListen>> = Vec::new();

        for listen in value.into_iter() {
            data.push(Rc::new(listen.try_into()?))
        }

        Ok(Self { data })
    }
}
