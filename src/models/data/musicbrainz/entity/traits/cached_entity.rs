pub trait CachedEntity {
    fn get_cache() -> MusicbrainzCache<Self>;
}
