use derive_getters::Getters;

use super::MBID;

#[derive(Debug, Clone, Getters)]
pub struct MBIDMerge<T: Into<MBID>> {
    from: T,
    to: T,
}
