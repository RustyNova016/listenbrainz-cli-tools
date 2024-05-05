use std::fmt::Display;
use std::sync::Arc;

use chashmap::CHashMap;
use tokio::sync::Semaphore;

#[derive(Debug)]
pub struct FetchLimiter {
    semaphores: CHashMap<String, Arc<Semaphore>>,
}

impl Default for FetchLimiter {
    fn default() -> Self {
        Self {
            semaphores: CHashMap::new(),
        }
    }
}

impl FetchLimiter {
    pub fn get_semaphore<K: Display>(&self, key: &K) -> Arc<Semaphore> {
        if let Some(semaphore) = self.semaphores.get(&key.to_string()) {
            return (*semaphore).clone();
        }

        self.semaphores
            .insert(key.to_string(), Arc::new(Semaphore::new(1)));
        return (*self
            .semaphores
            .get(&key.to_string())
            .expect("Couldn't get a new semaphore"))
        .clone();
    }
}
