use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

pub(crate) static RUNTIME_CONFIG: Lazy<Arc<RwLock<RuntimeConfig>>> = Lazy::new(|| Arc::new(RwLock::new(RuntimeConfig::default())));


/// Configuration at runtime. 
pub struct RuntimeConfig {
    pub primary_mbids_check: bool,
    pub update_listens: bool,
    pub update_unmapped_listens: bool
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            primary_mbids_check: true,
            update_listens: true,
            update_unmapped_listens: false
        }
    }
}