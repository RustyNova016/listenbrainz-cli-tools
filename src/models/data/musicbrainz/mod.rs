
pub mod artist;
pub mod artist_credit;
pub mod external_musicbrainz_entity;
pub mod mbid;
pub mod musicbrainz_entity;
pub mod recording;
pub mod relation;
pub mod release;
pub mod work;
pub mod release_group;

/// Type of the entity having this MBID
#[derive(Debug, Clone, Copy)]
pub enum MBIDType {
    Recording,
    Artist,
}
pub trait HasMbid {
    fn get_mbid(&self) -> &str;
}

pub trait HasId {
    fn get_id(&self) -> &str;
}

impl<T: HasMbid> HasId for T {
    fn get_id(&self) -> &str {
        self.get_mbid()
    }
}
