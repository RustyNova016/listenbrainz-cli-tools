use color_eyre::{eyre::Ok, Result};

use crate::models::data::musicbrainz::HasMbid;
use std::sync::Arc;

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
    fn insert_ms_with_id_into_cache(mbid: String, value: MbV) -> Result<()>;

    fn insert_ms_with_id_iter_into_cache<I: IntoIterator<Item = (String, MbV)>>(
        values: I,
    ) -> Result<()> {
        values
            .into_iter().try_for_each(|(mbid, value)| Self::insert_ms_with_id_into_cache(mbid, value))
    }
}

pub trait CacheFromMusicbrainzAutoId<MbV>: CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self> + HasMbid + Clone,
    Self: Sized,
{
    /// Insert the current item with its own MBID into the cache.
    fn insert_ms_into_cache(value: MbV) -> Result<()> {
        Self::insert_ms_with_id_into_cache(value.get_mbid().to_string(), value)
    }

    /// Insert a collection of items with their own MBIDs into the cache.
    fn insert_ms_iter_into_cache<I: IntoIterator<Item = MbV>>(values: I) -> Result<()> {
        values
            .into_iter().try_for_each(|value| Self::insert_ms_into_cache(value))
    }

    /// Insert the current item with its own MBID into the cache, as well as an alias MBID.
    /// This is useful incase an item has be been merged, and the alias MBID is only a reference to the original.
    fn insert_ms_with_alias_into_cache(alias_mbid: String, value: MbV) -> Result<()> {
        Self::insert_ms_into_cache(value.clone())?;
        Self::insert_ms_with_id_into_cache(alias_mbid, value)
    }
}

impl<V, MbV> CacheFromMusicbrainzAutoId<MbV> for V
where
    MbV: Into<V> + HasMbid + Clone,
    V: Sized + CacheFromMusicbrainz<MbV>,
{
}
