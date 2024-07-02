use crate::models::data::musicbrainz::mbid::MBID;
use std::io;
use thiserror::Error;

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
}

impl Error {}
