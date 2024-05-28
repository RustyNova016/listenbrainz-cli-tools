use clap::ValueEnum;
use derive_more::IsVariant;
use std::fmt::Display;

#[derive(ValueEnum, Clone, Debug, Copy, Default, IsVariant)]
pub enum SortListensBy {
    #[default]
    None,
    Name,
    OldestListen,
}

#[derive(ValueEnum, Clone, Debug, Copy, Default, IsVariant)]
pub enum SortSorterBy {
    /// The count of listens for this element. This is descending by default
    #[default]
    Count,

    /// The name of the associated element
    Name,

    /// The oldest element
    Oldest,
    // /// The oldest listen
}

impl Display for SortSorterBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::Name => write!(f, "name"),
            Self::Oldest => write!(f, "oldest"),
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum GroupByTarget {
    Recording,
    Artist,
    Release,
    ReleaseGroup,
    Work,
}

impl GroupByTarget {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Recording => "recording",
            Self::Artist => "artist",
            Self::Release => "release",
            Self::ReleaseGroup => "release_group",
            Self::Work => "work",
        }
    }
}
