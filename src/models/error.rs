use crate::core::caching::serde_cacache;
use crate::models::data::musicbrainz::mbid::MBID;
use std::io;
use thiserror::Error;

use super::data::musicbrainz::mbid::state_id::state::PrimaryMBID;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    /// Returned when an index was targeted to alias another of a different type
    #[error("MBID {1:?} couldn't be aliased to MBID {0:?}")]
    MBIDAliasError(MBID, MBID),

    #[error("Couldn't parse the string for any MBID. If you are sure that there is one, but see this error, please send a ticket.")]
    MBIDStringParsingError,

    // --- Config Errors ---
    #[error("An error occured when trying to load the configuration file.")]
    ConfigLoadError(io::Error),

    #[error("Couldn't load the configuration file. The configuration schema is incorrect")]
    ConfigLoadDeserializationError(serde_json::Error),

    #[error("Couldn't create the configuration file.")]
    ConfigFileCreationError(io::Error),

    #[error("Couldn't write the configuration file.")]
    ConfigFileWriteError(serde_json::Error),

    // --- Caching --- //
    #[error("Error while getting the cache")]
    CacheError(#[from] serde_cacache::error::Error),

    /// This error is made when trying to fetch an entity with a known redirection. The value contained is new MBID of the entity
    #[error("Tried to fetch a redirected MBID. {0:?} should be fetched instead")]
    MBIDRedirectError(String),

    // --- Fetching --- //
    #[error("Cannot find entity in Musicbrainz. It may have been deleted, or simply may not exist at all")]
    MBNotFound(String),

    // --- Type errors --- //
    #[error("Couldn't convert {0} into {1}")]
    InvalidTypeConvertion(String, String),

    #[error("Tried to convert from entity kind {0} into {1}")]
    InvalidKindConvertion(String, String),
}

impl Error {}
