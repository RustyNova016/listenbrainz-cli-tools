use std::ops::Deref;

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
            Self::Recording(data) => data,
            Self::Artist(data) => data,
        }
    }
}
