use crate::models::data::musicbrainz::mbid::MBID;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Returned when an index was targeted to alias another of a different type
    #[error("MBID {1:?} couldn't be aliased to MBID {0:?}")]
    MBIDAliasError(MBID, MBID),
}

impl Error {}
