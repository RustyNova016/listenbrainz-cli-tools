use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::error::Error;

use super::Artist;

impl Artist {
    pub fn try_from_entity(value: MusicBrainzEntity) -> Result<Self, Error> {
        Self::try_from(value)
    }
}

impl TryFrom<MusicBrainzEntity> for Artist {
    type Error = Error;

    fn try_from(value: MusicBrainzEntity) -> Result<Self, Self::Error> {
        if let MusicBrainzEntity::Artist(val) = value {
            return Ok(val);
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "Artist".to_string(),
        ))
    }
}
