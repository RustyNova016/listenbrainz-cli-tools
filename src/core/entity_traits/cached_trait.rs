use crate::models::data::musicbrainz::HasMbid;

#[deprecated]
pub trait CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self>,
    Self: Sized,
{
    #[deprecated]
    /// Insert an element with a specific MBID into the cache
    fn insert_ms_with_id_into_cache(mbid: String, value: MbV) -> color_eyre::Result<()>;

    #[deprecated]
    /// Insert an iterator element with a specific MBID into the cache
    fn insert_ms_with_id_iter_into_cache<I: IntoIterator<Item = (String, MbV)>>(
        values: I,
    ) -> color_eyre::Result<()> {
        values
            .into_iter()
            .try_for_each(|(mbid, value)| Self::insert_ms_with_id_into_cache(mbid, value))
    }
}

#[deprecated]
pub trait CacheFromMusicbrainzAutoId<MbV>: CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self> + HasMbid + Clone,
    Self: Sized,
{
    #[deprecated]
    /// Insert the current item with its own MBID into the cache.
    fn insert_ms_into_cache(value: MbV) -> color_eyre::Result<()> {
        Self::insert_ms_with_id_into_cache(value.get_mbid().to_string(), value)
    }

    #[deprecated]
    /// Insert a collection of items with their own MBIDs into the cache.
    fn insert_ms_iter_into_cache<I: IntoIterator<Item = MbV>>(values: I) -> color_eyre::Result<()> {
        values
            .into_iter()
            .try_for_each(|value| Self::insert_ms_into_cache(value))
    }

    #[deprecated]
    /// Insert the current item with its own MBID into the cache, as well as an alias MBID.
    /// This is useful incase an item has be been merged, and the alias MBID is only a reference to the original.
    fn insert_ms_with_alias_into_cache(alias_mbid: String, value: MbV) -> color_eyre::Result<()> {
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
