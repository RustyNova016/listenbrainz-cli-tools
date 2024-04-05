use std::sync::Arc;
use color_eyre::Result;

pub trait Cached<K, V> {
    fn get_cached_or_fetch(key: &K) -> Result<Arc<V>> {
        let cached = Self::get_cached(key);
        
        match cached {
            Some(cached) => Ok(cached),
            None => Self::fetch(key)
        }
    }
    
    fn get_cached(key: &K) -> Option<Arc<V>>;
    
    fn fetch(key: &K) -> Result<Arc<V>>;
}