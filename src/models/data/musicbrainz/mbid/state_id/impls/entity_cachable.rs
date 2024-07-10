use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::entity::traits::cached_entity::CachedEntity;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::mbid::state_id::MusicBrainzEntity;

impl<T, S> MBIDWithState<T, S>
where
    T: MusicBrainzEntity + CachedEntity + ?Sized,
    S: MBIDState,
{
    pub fn get_entity_cache() -> MusicbrainzCache<T> {
        T::get_cache()
    }
}
