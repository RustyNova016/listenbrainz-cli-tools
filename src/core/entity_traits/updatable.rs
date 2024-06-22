use std::sync::Arc;

pub trait Updatable: Sized {
    fn update(self, newer: Self) -> Self {
        newer
    }
}

impl<T: Updatable + Clone> Updatable for Arc<T> {
    fn update(self, newer: Self) -> Self {
        self.as_ref()
            .to_owned()
            .update(newer.as_ref().to_owned())
            .into()
    }
}
