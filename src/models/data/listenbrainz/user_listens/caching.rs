use crate::models::cache::traits::merge::UpdateCachedEntity;

use super::UserListens;

impl UpdateCachedEntity for UserListens {
    fn update_entity(self, new: Self) -> Self {
        new
    }
}
