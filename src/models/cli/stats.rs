use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum GroupByTarget {
    Recording,
    Artist,
}
