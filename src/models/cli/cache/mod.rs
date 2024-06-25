use std::path::PathBuf;

use crate::models::cli::cache::load_listen_dump::load_listen_dump;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use clap::ValueEnum;
use clap::{Parser, Subcommand};
use futures::try_join;

pub mod load_listen_dump;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub command: CacheSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CacheSubcommands {
    /// Load a listen dump from the website
    LoadDump {
        /// Name of the user to import those listens for
        username: String,

        /// Path to the dump file
        path: PathBuf,
    },

    /// Wipe the cache's data
    Clear { target: ClearTarget },
}

impl CacheCommand {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match &self.command {
            CacheSubcommands::LoadDump { username, path } => {
                load_listen_dump(path, username).await?;
            }
            CacheSubcommands::Clear { target } => {
                let _ = try_join!(
                    MUSICBRAINZ_DATABASE.clear(target),
                    ENTITY_DATABASE.clear(*target)
                )?;
            }
        }

        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum ClearTarget {
    All,
}
