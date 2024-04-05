use std::{ops::Deref, sync::Arc};

use super::static_cache::{StaticCache, STATIC_CACHE};

#[derive(Debug, Clone)]
pub struct GlobalCache {
    inner: Arc<StaticCache>,
}

impl GlobalCache {
    pub fn new() -> Self {
        Self {
            inner: STATIC_CACHE.clone(),
        }
    }
}

impl Deref for GlobalCache {
    type Target = Arc<StaticCache>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
