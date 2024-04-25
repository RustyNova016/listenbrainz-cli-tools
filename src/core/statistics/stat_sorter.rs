use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::Arc;

pub trait StatSorter {
    fn get_map_mut(&mut self) -> &mut HashMap<String, ListenCollection>;

    fn into_vec(self) -> Vec<(String, ListenCollection)>;

    #[allow(async_fn_in_trait)] // This temporary until the sorters get redone
    async fn push(&mut self, value: Arc<Listen>) -> color_eyre::Result<()>;

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

    #[allow(async_fn_in_trait)] // This temporary until the sorters get redone
    async fn extend<T: IntoIterator<Item = Arc<Listen>>>(
        &mut self,
        iter: T,
    ) -> color_eyre::Result<()> {
        for element in iter {
            self.push(element).await?;
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
