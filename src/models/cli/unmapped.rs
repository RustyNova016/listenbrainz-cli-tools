use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum SortBy {
    Count,
    Name,
}
