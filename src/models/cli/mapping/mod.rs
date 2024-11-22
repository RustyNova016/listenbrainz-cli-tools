use clap::Parser;
use clap::Subcommand;

use crate::models::config::Config;
use crate::tools::interactive_mapper::interactive_mapper;
use crate::tools::listens::unlinked::unmapped_command;

use super::common::SortListensBy;
use super::common::SortSorterBy;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct MappingCommand {
    #[command(subcommand)]
    subcommand: MappingSubcommands,
}

impl MappingCommand {
    pub async fn run(&self) -> color_eyre::Result<()> {
        self.subcommand.run().await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum MappingSubcommands {
    /// List all of your unlinked listens
    ///
    /// This command will list all your unmapped listens, grouped by similarity.
    /// It also gives a link to quickly look up the listen in listenbrainz, and go link it
    ///
    /// ```text
    ///
    /// (1) Paul's Dream (Dune) - Caster
    ///     -> <https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553>
    ///
    /// (7) Raise Your Weapon - KLOUD
    ///     -> <https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522>
    ///
    /// Total: 8 unlinked recordings
    ///
    /// ```
    ///
    /// > Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens
    /// > by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
    ///
    /// > This also means that the same recording can be shown twice in the list.
    /// > For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.
    ListUnmapped {
        /// Name of the user to fetch unlinked listen from
        username: Option<String>,

        /// Sort the listens by type
        #[arg(short, long)]
        sort: Option<SortSorterBy>,
    },

    /// Easy and faster mapping of recordings.
    ///
    /// It goes through each unmapped recordings, and give a few suggested recordings for the mapping. This is the exact same as mapping recording in the web UI.
    Mapper {
        /// Name of the user to fetch listens from
        username: Option<String>,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,

        /// Sort the listens by type
        #[arg(short, long)]
        sort: Option<SortListensBy>,
    },
}

impl MappingSubcommands {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::ListUnmapped { username, sort } => {
                unmapped_command(&Config::check_username(username).to_lowercase(), *sort).await;
            }

            Self::Mapper {
                username,
                token,
                sort,
            } => {
                interactive_mapper(
                    &Config::check_username(username),
                    Config::check_token(&Config::check_username(username), token),
                    *sort,
                )
                .await?;
            }
        }

        Ok(())
    }
}
