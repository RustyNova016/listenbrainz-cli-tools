use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::error::Error;

use super::Artist;

impl From<MusicBrainzEntity> for Result<Artist, Error> {
    fn from(value: MusicBrainzEntity) -> Self {
        if let MusicBrainzEntity::Artist(val) = value {
            return Ok(val);
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "Artist".to_string(),
        ))
    }
}
