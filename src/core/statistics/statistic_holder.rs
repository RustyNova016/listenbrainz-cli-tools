use crate::models::data::listenbrainz::listen::Listen;
use std::sync::Arc;

pub trait StatisticHolder<K> {
    fn insert_listen(
        &self,
        listen: Arc<Listen>,
    ) -> impl std::future::Future<Output = color_eyre::Result<()>>;

    fn count(&self) -> usize;

    fn create(id: K) -> Self;
}
