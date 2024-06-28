#[deprecated]
pub trait Updatable: Sized {
    #[deprecated]
    fn update(self, newer: Self) -> Self {
        newer
    }
}
