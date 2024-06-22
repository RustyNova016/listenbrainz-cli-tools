use std::io;

use thiserror::Error;

use crate::core::caching::serde_cacache;
use crate::models::data::musicbrainz::mbid::MBID;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    /// Returned when an index was targeted to alias another of a different type
    #[error("MBID {1:?} couldn't be aliased to MBID {0:?}")]
    MBIDAliasError(MBID, MBID),

    #[error("Couldn't parse the string for any MBID. If you are sure that there is one, but see this error, please send a ticket.")]
    MBIDStringParsingError,

    // --- Config Errors --- //
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
    CacheError(serde_cacache::error::Error),

    // --- Fetching Errors --- //
    #[error("Tried to fetch the {0} field is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \
    Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {1}"
    )]
    NoneAfterFetch(String, String),
}

impl Error {}
