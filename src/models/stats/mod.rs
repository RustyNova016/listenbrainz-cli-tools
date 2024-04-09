use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::Arc;

use color_eyre::Result;

use super::data::listenbrainz::listen::collection::ListenCollection;
use super::data::listenbrainz::listen::Listen;

pub mod artist_stats;
pub mod recording_stats;
pub mod stat_item;

pub trait StatSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, ListenCollection>;

    fn into_vec(self) -> Vec<(String, ListenCollection)>;

    fn push(&mut self, value: Arc<Listen>) -> Result<()>;

    fn get_mut(&mut self, key: &String) -> &mut ListenCollection {
        if self.get_map_mut().get(key).is_none() {
            // No vec at this location. So we add one and return it
            self.get_map_mut()
                .insert(key.clone(), ListenCollection::new());
        }

        return self
            .get_map_mut()
            .get_mut(key)
            .expect("Could not retrieve EntityStats from stat list");
    }

    fn extend<T: IntoIterator<Item = Arc<Listen>>>(&mut self, iter: T) -> Result<()> {
        for element in iter {
            self.push(element)?;
        }

        Ok(())
    }

    fn into_sorted(self) -> Vec<(String, ListenCollection)>
    where
        Self: Sized,
    {
        let mut out = self.into_vec();
        out.sort_unstable_by_key(|item| Reverse(item.1.len()));
        out
    }
}
