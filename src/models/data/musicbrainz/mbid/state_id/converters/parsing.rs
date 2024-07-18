use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::any::any_id_to_kind::AnyIdToKind;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveIDState;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::error::Error;
use crate::utils::regex::is_string_mbid;
use crate::utils::regex::parse_mbid_from_url;

impl<T> NaiveMBID<T>
where
    T: MusicBrainzEntity,
{
    pub fn parse_string(value: String) -> Result<Self, Error> {
        if is_string_mbid(&value) {
            Ok(Self::from(value))
        } else {
            Err(Error::MBIDStringParsingError)
        }
    }

    pub fn parse_url(value: String) -> Result<Self, Error>
    where
        Self: AnyIdToKind<T, NaiveIDState>,
    {
        let Some(parsed) = parse_mbid_from_url(&value) else {
            return Err(Error::MBIDStringParsingError);
        };

        Self::try_from_any(parsed)
    }

    pub fn parse_string_or_url(value: String) -> Result<Self, Error>
    where
        Self: AnyIdToKind<T, NaiveIDState>,
    {
        Self::parse_string(value.clone()).or_else(|_| Self::parse_url(value))
    }
}
