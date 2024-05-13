use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Storage error")]
    CacheError(#[from] cacache::Error),

    #[error("Error deserializing cached value")]
    CacheDeserializationError(#[from] rmp_serde::decode::Error),

    #[error("Error serializing cached value")]
    CacheSerializationError(#[from] rmp_serde::encode::Error),
}
