use crate::models::cache::{
    global_cache::GlobalCache,
    traits::{has_cache::HasCache, merge::UpdateCachedEntity},
};

use super::UserListens;

impl UpdateCachedEntity for UserListens {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}

impl HasCache<String, UserListens> for UserListens {
    fn get_cache(
    ) -> std::sync::Arc<crate::models::cache::disk_cache::DiskCacheWrapper<String, UserListens>>
    {
        GlobalCache::new().get_listen_cache()
    }
}
