use std::sync::Arc;

use crate::models::data::musicbrainz::entity::any_musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::error::Error;

use super::Artist;

impl From<AnyMusicBrainzEntity> for Result<Arc<Artist>, Error> {
    fn from(value: AnyMusicBrainzEntity) -> Self {
        if let AnyMusicBrainzEntity::Artist(val) = value {
            return Ok(val);
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "Artist".to_string(),
        ))
    }
}
