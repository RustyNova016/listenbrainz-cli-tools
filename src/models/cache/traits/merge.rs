pub trait UpdateCachedEntity {
    fn update_entity(self, new: Self) -> Self;
}
