use crate::models::api::musicbrainz::MusicBrainzAPI;
use std::{collections::HashMap, rc::Rc};

use super::data::listens::UserListen;

pub mod artist_stats;
pub mod recording_stats;
pub mod stat_struct;

pub trait StatSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, Vec<Rc<UserListen>>>;

    fn push(&mut self, value: Rc<UserListen>, mb_client: &mut MusicBrainzAPI);

    fn get_mut(&mut self, key: &String) -> &mut Vec<Rc<UserListen>> {
        if self.get_map_mut().get(key).is_none() {
            // No vec at this location. So we add one and return it
            self.get_map_mut().insert(key.clone(), Vec::new());
        }

        return self
            .get_map_mut()
            .get_mut(key)
            .expect("Could not retrieve EntityStats from stat list");
    }

    fn extend<T: IntoIterator<Item = Rc<UserListen>>>(
        &mut self,
        iter: T,
        mb_client: &mut MusicBrainzAPI,
    ) {
        for element in iter {
            self.push(element, mb_client)
        }
    }

    fn into_sorted(self) -> Vec<Vec<Rc<UserListen>>>;
}
