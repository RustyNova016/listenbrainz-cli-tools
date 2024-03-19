use std::{collections::HashMap, rc::Rc};

use super::{data::listens::UserListen, musicbrainz::MBIDType};

pub struct MBIDStatCounter {
    data: HashMap<String, Vec<Rc<UserListen>>>,
    mbid_type: MBIDType,
}

impl MBIDStatCounter {
    /// Add a listen in the stat counter
    pub fn add_listen(&mut self, item: Rc<UserListen>) {
        match self.mbid_type {
            MBIDType::Recording => self.add_listen_recording(item),
        }
    }

    /// Add a listen depending on the recording MBID
    fn add_listen_recording(&mut self, item: Rc<UserListen>) {
        let Some(mbid_mapping) = item.get_mapping_data() else {
            return;
        };

        if let Some(listens) = self.data.get_mut(mbid_mapping.get_recording_id()) {
            listens.push(item);
        } else {
            self.data
                .insert(mbid_mapping.get_recording_id().clone(), vec![item]);
        }
    }
}
