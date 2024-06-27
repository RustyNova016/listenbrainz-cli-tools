/// Converts a reference of `self` into a reference of `T`
pub trait AsType<T> {
    fn as_type(&self) -> &T;
}

/// Converts a reference of `self` into a reference of `T` if it can, or else return `None`
pub trait AsOptionalType<T> {
    fn as_optional_type(&self) -> Option<&T>;
}

pub trait OptionalFrom<T>
where
    Self: Sized,
{
    fn optional_from(value: T) -> Option<Self>;
}

pub trait OptionalInto<U>
where
    Self: Sized,
    U: OptionalFrom<Self>,
{
    fn optional_into(self) -> Option<U> {
        U::optional_from(self)
    }
}
