use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Copy, Default)]
pub enum SortListensBy {
    #[default]
    None,
    Name,
    OldestListen,
}

#[derive(ValueEnum, Clone, Debug, Copy, Default)]
pub enum SortSorterBy {
    #[default]
    Count,
    Name,
    Oldest,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum GroupByTarget {
    Recording,
    Artist,
}
