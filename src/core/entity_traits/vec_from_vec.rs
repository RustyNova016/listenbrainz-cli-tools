use itertools::Itertools;

pub trait InnerFrom<T, U>
where
    Self: IntoIterator<Item = U> + Sized,
    T: From<U>,
{
    /// Convert any iterator of U into a Vec<T>
    fn inner_from(self) -> Vec<T> {
        self.into_iter().map(T::from).collect_vec()
    }
}

impl<T, U> InnerFrom<T, U> for Vec<U> where T: From<U> {}
