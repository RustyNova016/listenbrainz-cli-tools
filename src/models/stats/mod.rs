use crate::utils::traits::VecWrapper;
use std::{cmp::Reverse, collections::HashMap, sync::Arc};

use super::data::listens::{collection::UserListenCollection, UserListen};
use color_eyre::Result;

pub mod artist_stats;
pub mod recording_stats;
pub mod stat_struct;

pub trait StatSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, UserListenCollection>;

    fn into_vec(self) -> Vec<(String, UserListenCollection)>;

    fn push(&mut self, value: Arc<UserListen>) -> Result<()>;

    fn get_mut(&mut self, key: &String) -> &mut UserListenCollection {
        if self.get_map_mut().get(key).is_none() {
            // No vec at this location. So we add one and return it
            self.get_map_mut()
                .insert(key.clone(), UserListenCollection::new());
        }

        return self
            .get_map_mut()
            .get_mut(key)
            .expect("Could not retrieve EntityStats from stat list");
    }

    fn extend<T: IntoIterator<Item = Arc<UserListen>>>(&mut self, iter: T) -> Result<()> {
        for element in iter {
            self.push(element)?;
        }

        Ok(())
    }

    fn into_sorted(self) -> Vec<(String, UserListenCollection)>
    where
        Self: Sized,
    {
        let mut out = self.into_vec();
        out.sort_unstable_by_key(|item| Reverse(item.1.len()));
        out
    }
}
