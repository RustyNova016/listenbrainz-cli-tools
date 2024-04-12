use musicbrainz_rs::entity::release::Media as MediaMS;

use crate::models::{
    cache::{
        cached_trait::{CacheFromMusicbrainz, CacheFromMusicbrainzAutoId},
        traits::InsertExternalEntityIntoCache,
    },
    data::musicbrainz::{release::track::Track, HasId},
};

use super::Media;

impl InsertExternalEntityIntoCache<Media, MediaMS> for Media {
    fn insert_ext_into_cache(value: MediaMS) -> color_eyre::eyre::Result<()> {
        if let Some(tracks) = value.tracks {
            Track::insert_ms_iter_into_cache(tracks)?;
        }

        Ok(())
    }
}
