use crate::tools::{
    stats::{stats_command},
    unlinked::unlinked_command,
};
use clap::{Parser, Subcommand};

use self::stats::GroupByTarget;

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
    Unlinked {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,
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
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Unlinked { username } => unlinked_command(username),
            Commands::Stats { username, target } => stats_command(username, *target),
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
