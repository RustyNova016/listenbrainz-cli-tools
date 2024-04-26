use crate::core::statistics::statistic_holder::StatisticHolder;
use crate::models::data::listenbrainz::listen::Listen;
use std::cmp::Reverse;
use std::sync::Arc;

pub trait StatisticSorter<K, H: StatisticHolder<K>> {
    fn insert_listen(
        &self,
        listen: Arc<Listen>,
    ) -> impl std::future::Future<Output = color_eyre::Result<()>> + Send;

    fn get(&self, key: &K) -> Arc<H>;

    fn extend<'a, T: IntoIterator<Item = Arc<Listen>>>(
        &'a self,
        iter: T,
    ) -> impl std::future::Future<Output = color_eyre::Result<()>>
    where
        K: 'a,
        H: 'a,
    {
        async {
            for listen in iter.into_iter() {
                self.insert_listen(listen).await?;
            }

            Ok(())
        }
    }

    fn into_vec(self) -> Vec<(String, Arc<H>)>;

    fn into_sorted(self) -> Vec<(String, Arc<H>)>
    where
        Self: Sized,
    {
        let mut out = self.into_vec();
        out.sort_unstable_by_key(|item| Reverse(item.1.count()));
        out
    }
}