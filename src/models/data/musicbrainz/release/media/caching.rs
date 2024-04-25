use crate::core::entity_traits::cached_trait::CacheFromMusicbrainzAutoId;
use musicbrainz_rs::entity::release::Media as MediaMS;

use crate::models::data::musicbrainz::release::track::Track;

use super::Media;
use crate::core::entity_traits::insert_external_entity_into_cache::InsertExternalEntityIntoCache;

impl InsertExternalEntityIntoCache<Media, MediaMS> for Media {
    fn insert_ext_into_cache(value: MediaMS) -> color_eyre::eyre::Result<()> {
        if let Some(tracks) = value.tracks {
            Track::insert_ms_iter_into_cache(tracks)?;
        }

        Ok(())
    }
}
