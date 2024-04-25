pub trait InsertExternalEntityIntoCache<T, E: Into<T>> {
    fn insert_ext_into_cache(value: E) -> color_eyre::Result<()>;

    fn insert_ext_iter_into_cache<I: IntoIterator<Item = E>>(values: I) -> color_eyre::Result<()> {
        values
            .into_iter()
            .try_for_each(|value| Self::insert_ext_into_cache(value))
    }

    fn insert_opt_ext_into_cache(value: Option<E>) -> color_eyre::Result<()> {
        match value {
            Some(value) => Self::insert_ext_into_cache(value),
            None => Ok(()),
        }
    }

    fn insert_opt_ext_iter_into_cache<I: IntoIterator<Item = E>>(
        values: Option<I>,
    ) -> color_eyre::Result<()> {
        match values {
            Some(values) => Self::insert_ext_iter_into_cache(values),
            None => Ok(()),
        }
    }
}
