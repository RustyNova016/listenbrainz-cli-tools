use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub(crate) static MB_FETCH_LIMITER: Lazy<Arc<Semaphore>> =
    Lazy::new(|| Arc::new(Semaphore::new(1)));
