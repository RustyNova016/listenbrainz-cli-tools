use super::MBID;

#[derive(Debug, Clone, Copy)]
pub enum MBIDKind {
    Area,
    Artist,
    Event,
    Genre,
    Instrument,
    Label,
    Place,
    Recording,
    Release,
    ReleaseGroup,
    Series,
    URL,
    Work,
}

impl MBIDKind {
    pub fn to_mbid(&self, data: String) -> MBID {
        match self {
            Self::Artist => MBID::Artist(data.into()),
            Self::Recording => MBID::Recording(data.into()),
            Self::Release => MBID::Release(data.into()),
            Self::ReleaseGroup => MBID::ReleaseGroup(data.into()),
            Self::Work => MBID::Work(data.into()),
            _ => todo!(),
        }
    }
}

impl From<MBID> for MBIDKind {
    fn from(value: MBID) -> Self {
        match value {
            MBID::Artist(_) => Self::Artist,
            MBID::Recording(_) => Self::Recording,
            MBID::Release(_) => Self::Release,
            MBID::ReleaseGroup(_) => Self::ReleaseGroup,
            MBID::Work(_) => Self::Work,
        }
    }
}
