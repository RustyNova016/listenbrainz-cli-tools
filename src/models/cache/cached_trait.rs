use color_eyre::Result;

use std::sync::Arc;

use crate::models::musicbrainz::HasMbid;

pub trait CachedOLD<K, V> {
    fn get_cached_or_fetch(key: &K) -> Result<Arc<V>> {
        let cached = Self::get_cached(key);

        match cached {
            Some(cached) => Ok(cached),
            None => Self::fetch(key),
        }
    }

    fn get_cached(key: &K) -> Option<Arc<V>>;

    fn fetch(key: &K) -> Result<Arc<V>>;
}

pub trait CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self>,
    Self: Sized,
{
    fn insert_ms_with_id_into_cache(mbid: String, value: MbV);

    fn insert_ms_with_id_iter_into_cache<I: IntoIterator<Item = (String, MbV)>>(values: I) {
        values
            .into_iter()
            .for_each(|(mbid, value)| Self::insert_ms_with_id_into_cache(mbid, value))
    }
}

pub trait CacheFromMusicbrainzAutoId<MbV>: CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self> + HasMbid,
    Self: Sized,
{
    fn insert_ms_into_cache(value: MbV) {
        Self::insert_ms_with_id_into_cache(value.get_mbid().to_string(), value)
    }

    fn insert_ms_iter_into_cache<I: IntoIterator<Item = MbV>>(values: I) {
        values
            .into_iter()
            .for_each(|value| Self::insert_ms_into_cache(value))
    }
}

impl<V, MbV> CacheFromMusicbrainzAutoId<MbV> for V
where
    MbV: Into<V> + HasMbid,
    V: Sized + CacheFromMusicbrainz<MbV>,
{
}
