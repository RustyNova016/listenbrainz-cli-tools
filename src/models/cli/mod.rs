pub mod cache;
use cache::CacheCommand;
use clap::{Parser, Subcommand};
use config::ConfigCli;

use crate::models::cli::common::{GroupByTarget, SortListensBy, SortSorterBy};
use crate::models::cli::radio::RadioCommand;
use crate::tools::interactive_mapper::interactive_mapper;
use crate::tools::stats::stats_command;
use crate::tools::unlinked::unmapped_command;

use super::config::Config;

pub mod common;
pub mod config;
pub mod radio;

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

        /// Sort by:
        #[arg(short, long, default_value_t = SortSorterBy::Count)]
        sort: SortSorterBy,
    },

    /// Map unmapped recordings easily
    Mapping {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: Option<String>,

        /// Sort the listens by type
        #[arg(short, long)]
        sort: Option<SortListensBy>,
    },

    /// Generate playlists
    Radio(RadioCommand),

    Cache(CacheCommand),
    Config(ConfigCli),
    //Search {},

    //Lookup {
    //    /// Recording ID
    //    #[arg(short, long)]
    //    id: String,

    //    /// Name of the user to fetch stats listen from
    //    #[arg(short, long)]
    //    username: String,
    //},
}

impl Commands {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::Unmapped { username, sort } => {
                unmapped_command(&username.to_lowercase(), *sort).await;
            }
            Self::Stats {
                username,
                target,
                sort,
            } => {
                stats_command(&username.to_lowercase(), *target, *sort).await;
            }
            Self::Mapping {
                username,
                token,
                sort,
            } => {
                interactive_mapper(
                    username,
                    Config::get_token_or_argument(username, token),
                    *sort,
                )
                .await?;
            }

            Self::Radio(val) => val.run().await?,
            Self::Cache(val) => val.run().await?,
            Self::Config(val) => val.command.run().await?,
            //Self::Lookup { id, username } => lookup(username, id.to_string().into()).await,
        }

        Ok(())
    }
}
