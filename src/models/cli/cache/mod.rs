use std::path::PathBuf;

use crate::models::cli::cache::load_listen_dump::load_listen_dump;
use clap::{Parser, Subcommand};

pub mod load_listen_dump;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub command: CacheSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CacheSubcommands {
    LoadDump {
        /// Name of the user to import those listens for
        username: String,

        /// Path to the dump file
        path: PathBuf,
    },
}

impl CacheCommand {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match &self.command {
            CacheSubcommands::LoadDump { username, path } => {
                load_listen_dump(path, username).await?;
            }
        }

        Ok(())
    }
}
