use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::error::ErrorKind;
use crate::utils::regex::get_mbid_from_url;
use crate::utils::regex::is_string_mbid;

use super::MBID;

impl MBID {
    pub fn from_string(value: &str, assumed_type: MusicbrainzEntityKind) -> Result<Self, ErrorKind> {
        Self::from_string_url(value).or_else(|_| Self::from_mbid_string(value, assumed_type))
    }

    fn from_mbid_string(value: &str, assumed_type: MusicbrainzEntityKind) -> Result<Self, ErrorKind> {
        if is_string_mbid(value) {
            Ok(assumed_type.to_mbid(value.to_string()))
        } else {
            Err(ErrorKind::MBIDStringParsingError)
        }
    }

    pub fn from_string_url(value: &str) -> Result<Self, ErrorKind> {
        get_mbid_from_url(value).ok_or(ErrorKind::MBIDStringParsingError)
    }
}
