use crate::models::error::Error;
use crate::utils::regex::get_mbid_from_url;
use crate::utils::regex::is_string_mbid;

use super::mbid_kind::MBIDKind;
use super::MBID;

impl MBID {
    pub fn from_string(value: &str, assumed_type: MBIDKind) -> Result<Self, Error> {
        Self::from_string_url(value).or_else(|_| Self::from_mbid_string(value, assumed_type))
    }

    fn from_mbid_string(value: &str, assumed_type: MBIDKind) -> Result<Self, Error> {
        if is_string_mbid(value) {
            Ok(assumed_type.to_mbid(value.to_string()))
        } else {
            Err(Error::MBIDStringParsingError)
        }
    }

    pub fn from_string_url(value: &str) -> Result<Self, Error> {
        get_mbid_from_url(value).ok_or(Error::MBIDStringParsingError)
    }
}
