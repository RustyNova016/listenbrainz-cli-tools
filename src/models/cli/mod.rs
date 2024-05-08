use clap::ArgAction;
use clap::{Parser, Subcommand};

use crate::models::cli::common::{GroupByTarget, SortListensBy, SortSorterBy};
use crate::tools::interactive_mapper::interactive_mapper;
use crate::tools::radio::create_radio_mix;
use crate::tools::stats::stats_command;
use crate::tools::unlinked::unmapped_command;

pub mod common;

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
        sort: Option<SortSorterBy>,
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

        /// User token
        #[arg(short, long)]
        token: String,

        /// Sort the listens by type
        #[arg(short, long)]
        sort: Option<SortListensBy>,
    },

    /// Generate playlists
    Radio {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: String,

        /// Use this flag to only get unlistened recordings
        #[clap(long, action=ArgAction::SetTrue)]
        unlistened: bool,
    },
}

impl Commands {
    pub async fn run(&self) {
        match self {
            Commands::Unmapped { username, sort } => {
                unmapped_command(&username.to_lowercase(), *sort).await;
            }
            Commands::Stats { username, target } => {
                stats_command(&username.to_lowercase(), *target).await;
            }
            Commands::Mapping {
                username,
                token,
                sort,
            } => interactive_mapper(username, token.clone(), *sort).await,

            Commands::Radio {
                username,
                token,
                unlistened,
            } => create_radio_mix(username, token.clone(), *unlistened).await,
        }
    }
}
