use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Storage error")]
    CacheError(#[from] cacache::Error),

    /// Returned when an index entry could not be found during
    /// lookup.
    #[error("Entry not found for key {1:?} in cache {0:?}")]
    EntryNotFound(PathBuf, String),

    #[error("Error deserializing cached value")]
    CacheDeserializationError(#[from] rmp_serde::decode::Error),

    #[error("Error serializing cached value")]
    CacheSerializationError(#[from] rmp_serde::encode::Error),
}

impl Error {}
