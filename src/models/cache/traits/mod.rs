use color_eyre::Result;

use crate::models::data::musicbrainz::HasId;

pub mod merge;

pub trait InsertExternalEntityIntoCache<T, E: Into<T>> {
    fn insert_ext_into_cache(value: E) -> Result<()>;

    fn insert_ext_iter_into_cache<I: IntoIterator<Item = E>>(values: I) -> Result<()> {
        values
            .into_iter()
            .map(|value| Self::insert_ext_into_cache(value))
            .collect()
    }

    fn insert_opt_ext_into_cache(value: Option<E>) -> Result<()> {
        match value {
            Some(value) => Self::insert_ext_into_cache(value),
            None => Ok(()),
        }
    }

    fn insert_opt_ext_iter_into_cache<I: IntoIterator<Item = E>>(values: Option<I>) -> Result<()> {
        match values {
            Some(values) => Self::insert_ext_iter_into_cache(values),
            None => Ok(()),
        }
    }
}
