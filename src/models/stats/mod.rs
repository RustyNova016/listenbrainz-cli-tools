use std::{
    collections::{hash_map::Values, HashMap},
    rc::Rc,
};

use super::{data::listens::UserListen, musicbrainz::MBIDType};

pub mod artist_stats;

pub struct MBIDStatCounter {
    data: HashMap<String, Vec<Rc<UserListen>>>,
    mbid_type: MBIDType,
}

impl MBIDStatCounter {
    pub fn new(mbid_type: MBIDType) -> Self {
        Self {
            mbid_type,
            data: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, key: &String) -> &mut Vec<Rc<UserListen>> {
        if self.data.get(key).is_none() {
            // No vec at this location. So we add one and return it
            self.data.insert(key.clone(), Vec::new());
        }

        return self
            .data
            .get_mut(key)
            .expect("Could not retrieve vector from stat list");
    }

    /// Add a listen in the stat counter
    pub fn push(&mut self, item: Rc<UserListen>) {
        match self.mbid_type {
            MBIDType::Recording => self.add_listen_recording(item),
            MBIDType::Artist => self.add_listens_artist(item),
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

    /// Add a listen depending on the artist MBID
    fn add_listens_artist(&mut self, item: Rc<UserListen>) {
        let Some(mbid_mapping) = item.get_mapping_data() else {
            return;
        };

        for artist_id in mbid_mapping.get_artists_mbids() {
            self.get_mut(artist_id).push(item.clone())
        }
    }

    pub fn values(&self) -> Values<'_, String, Vec<Rc<UserListen>>> {
        self.data.values()
    }

    pub fn into_sorted(self) -> Vec<Vec<Rc<UserListen>>> {
        let mut out = Vec::new();
        out.extend(self.data.into_values());
        out.sort_unstable_by_key(|item| item.len());
        out
    }
}

impl Extend<Rc<UserListen>> for MBIDStatCounter {
    fn extend<T: IntoIterator<Item = Rc<UserListen>>>(&mut self, iter: T) {
        for element in iter {
            self.push(element)
        }
    }
}
