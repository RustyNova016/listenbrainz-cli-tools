pub mod unmapped;
use crate::tools::{interactive_mapper::interactive_mapper, stats::stats_command};
use crate::tools::unlinked::unmapped_command;
use clap::{Parser, Subcommand};

use self::{stats::GroupByTarget, unmapped::SortBy};

pub mod stats;

/// Tools for Listenbrainz
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Tools with the unlinked listens
    Unmapped {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// Sort the listens by type
        #[arg(short, long)]
        sort: Option<SortBy>,
    },

    /// Live and accurate statistics
    Stats {
        //#[command(subcommand)]
        //command: StatsCommand,
        /// The type of entity to sort by.
        #[arg(short, long)]
        target: GroupByTarget,

        /// Name of the user to fetch stats listen from
        #[arg(short, long)]
        username: String,
    },

    /// Map unmapped recordings easily
    Mapping {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,
    },
}

impl Commands {
    pub async fn run(&self) {
        match self {
            Commands::Unmapped { username, sort } => {
                unmapped_command(&username.to_lowercase(), *sort)
            }
            Commands::Stats { username, target } => {
                stats_command(&username.to_lowercase(), *target).await
            }
            Commands::Mapping { username } => {interactive_mapper(username).await}
        }
    }
}

//#[derive(Subcommand, Debug, Clone)]
//pub enum StatsCommand {
//    /// Get recording stats (Default)
//    Recordings {
//
//    },
//
//    /// Get artist stats.
//    Artist {
//        /// Name of the user to fetch stats listen from
//        #[arg(short, long)]
//        username: String,
//    },
//}
//
//impl StatsCommand {
//    pub fn run(&self) {
//        match self {
//            StatsCommand::Recordings { username } => recording_stats(username),
//            StatsCommand::Artist { username } => artist_stats(username),
//        }
//    }
//}
