use clap::ValueEnum;
use derive_more::IsVariant;

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

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum GroupByTarget {
    Recording,
    Artist,
    Release,
    ReleaseGroup,
}
