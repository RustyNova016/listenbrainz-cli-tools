use std::future::Future;

pub trait FetchAPI<K, V> {
    /// Fetch an item a put it into the cache
    ///
    /// This operation isn't deduplicated! Refer to the Diskcache for safe call
    fn fetch_and_insert(key: &K) -> impl Future<Output = color_eyre::Result<V>>;
}
