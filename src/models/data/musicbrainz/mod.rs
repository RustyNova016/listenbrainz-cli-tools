use std::ops::Deref;

pub mod artist;
pub mod artist_credit;
pub mod recording;
pub mod release;
pub mod release_group;

/// Type of the entity having this MBID
#[derive(Debug, Clone, Copy)]
pub enum MBIDType {
    Recording,
    Artist,
}

#[derive(Debug, Clone)]
pub enum MBID {
    Recording(String),
    Artist(String),
}

impl Deref for MBID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Artist(data) | Self::Recording(data) => data,
        }
    }
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
