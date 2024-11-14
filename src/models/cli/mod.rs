use std::io;

use cache::CacheCommand;
use clap::Command;
use clap::CommandFactory;
use clap::{Parser, Subcommand};
use clap_complete::generate;
use clap_complete::Generator;
use clap_complete::Shell;
use config::ConfigCli;
use lookup::LookupCommand;
use mapping::MappingCommand;

use crate::models::cli::common::{GroupByTarget, SortSorterBy};
use crate::models::cli::radio::RadioCommand;
use crate::tools::bumps::bump_command;
use crate::tools::bumps::bump_down_command;
use crate::tools::compatibility::compatibility_command;
use crate::tools::stats::stats_command;

use super::config::Config;

pub mod cache;
pub mod common;
pub mod config;
pub mod lookup;
pub mod mapping;
pub mod radio;

/// Tools for Listenbrainz
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, hide = true)]
    pub markdown_help: bool,

    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub async fn run(&self) -> color_eyre::Result<bool> {
        // Invoked as: `$ my-app --markdown-help`
        if self.markdown_help {
            clap_markdown::print_help_markdown::<Self>();
            return Ok(false);
        }

        if let Some(generator) = self.generator {
            let mut cmd = Self::command();
            Self::print_completions(generator, &mut cmd);
            return Ok(false);
        }

        if let Some(command) = &self.command {
            command.run().await?;
        }

        Ok(true)
    }

    fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    }
}

/// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
///
/// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
///
/// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
#[derive(Parser, Debug, Clone)]
pub struct BumpCLI {
    /// The recording to bump
    pub recording: Option<String>,

    /// The duration the bump last for (Default: 3 months)
    #[arg(short, long)]
    pub duration: Option<String>,

    /// The multiplier added to the score (Default: 1.1)
    #[arg(short, long)]
    pub multiplier: Option<String>,

    #[arg(short, long)]
    pub username: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Bump(BumpCLI),

    /// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
    ///
    /// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
    ///    
    /// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
    BumpDown(BumpCLI),

    /// Commands to deal with the local cache
    Cache(CacheCommand),

    Compatibility {
        /// The name of the first user
        user_a: String,

        /// The name of the second user
        user_b: String,
    },

    /// Commands to deal with the app's configuration
    Config(ConfigCli),

    /// Get detailled information about an entity
    Lookup(LookupCommand),

    /// Commands for interacting with listen mappings
    Mapping(MappingCommand),

    /// Generate radio playlists for you
    Radio(RadioCommand),

    /// Shows top statistics for a specific target
    ///
    /// Target is the entity type to group the stats by. Currently, those entities stats are implemented:
    ///
    /// - Recordings (`recording`)
    ///
    /// - Artists (`artist`)
    ///
    /// - Releases (`release`)
    ///
    /// - Release Groups (`release_group`)
    ///
    /// - Works (`work`)
    Stats {
        //#[command(subcommand)]
        //command: StatsCommand,
        /// The type of entity to sort by.
        target: GroupByTarget,

        /// Name of the user to fetch stats listen from
        username: Option<String>,

        /// Sort by:
        #[arg(short, long, default_value_t = SortSorterBy::Count)]
        sort: SortSorterBy,
    },
}

impl Commands {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::Stats {
                username,
                target,
                sort,
            } => {
                stats_command(
                    &Config::check_username(username).to_lowercase(),
                    *target,
                    *sort,
                )
                .await;
            }

            Self::Compatibility { user_a, user_b } => compatibility_command(user_a, user_b).await,

            Self::Radio(val) => val.run().await?,

            Self::Cache(val) => val.run().await?,

            Self::Config(val) => val.command.run().await?,

            Self::Lookup(val) => val.run().await?,

            Self::Mapping(val) => val.run().await?,

            Self::Bump(val) => bump_command(val.clone()).await,

            Self::BumpDown(val) => bump_down_command(val.clone()).await,
        }
        Ok(())
    }
}
