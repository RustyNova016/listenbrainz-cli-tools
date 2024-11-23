use std::path::PathBuf;

use crate::database::get_conn;
//use crate::models::config::Config;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::tools::cache::delete_database;
//use crate::tools::listens::import::import_listen_dump;
use clap::ValueEnum;
use clap::{Parser, Subcommand};
use color_eyre::owo_colors::OwoColorize;
use futures::try_join;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub command: CacheSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CacheSubcommands {
    /// Initialise the database.
    InitDatabase {
        /// Wipe the database file beforehand
        #[arg(long)]
        reset: bool,
    },

    /// Load a listen dump from the website
    ///
    /// Allows to load an exported dump of you listens. This is often faster than using the app.
    /// This also prevent stumbling into LB-1584
    ///
    /// You can get a listen dump [here](https://listenbrainz.org/settings/export/)
    LoadDump {
        /// Path to the dump file
        path: PathBuf,

        /// Name of the user to import those listens for
        username: Option<String>,
    },

    /// Wipe the cache's data
    ///
    /// This is useful if you need disk space, or need to manually rebuild in case of corruption
    Clear { target: ClearTarget },
}

impl CacheCommand {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match &self.command {
            CacheSubcommands::InitDatabase { reset } => {
                if *reset {
                    delete_database().expect("Failed to delete the database");
                }
                get_conn().await;
            }
            CacheSubcommands::LoadDump {
                username: _,
                path: _,
            } => {
                println!();
                println!("   {}", "Temporary unavailable :(".black().on_red().bold());
                println!("The data dumps currently don't have enough information to be useful, and may give false informations.");
                println!("So the feature is temporarily disabled");
                println!();
                println!("Please see: https://tickets.metabrainz.org/browse/LB-1687");
                //import_listen_dump(path, &Config::check_username(username)).await;
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
