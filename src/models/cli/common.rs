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
    #[default]
    Count,
    Name,
    Oldest,
}

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum GroupByTarget {
    Recording,
    Artist,
}
