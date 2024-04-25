use crate::core::caching::disk_cache::DiskCacheWrapper;
use crate::core::caching::global_cache::GlobalCache;
use crate::core::entity_traits::has_cache::HasCache;
use crate::core::entity_traits::merge::UpdateCachedEntity;
use std::sync::Arc;

use super::UserListens;

impl UpdateCachedEntity for UserListens {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}

impl HasCache<String, UserListens> for UserListens {
    fn get_cache() -> Arc<DiskCacheWrapper<String, UserListens>> {
        GlobalCache::new().get_listen_cache()
    }
}
