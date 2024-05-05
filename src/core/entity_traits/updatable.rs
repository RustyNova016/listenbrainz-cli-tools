pub trait Updatable: Sized {
    fn update(self, newer: Self) -> Self {
        newer
    }
}
