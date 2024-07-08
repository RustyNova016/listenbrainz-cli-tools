use crate::models::cli::common::ConfigBool;
use crate::models::config::global_config::CONFIG;
use crate::utils::println_cli;
use clap::command;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ListenConfigCli {
    #[command(subcommand)]
    subcommand: ListenConfigSubcommands,
}

impl ListenConfigCli {
    pub async fn run(&self) -> color_eyre::Result<()> {
        self.subcommand.run().await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListenConfigSubcommands {
    /// Toggle / Set whether the unmapped listens should be automatically updated when fetching listens
    RefreshUnmappedListens {
        /// What do you want it set to?
        state: ConfigBool,
    },
}

impl ListenConfigSubcommands {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::RefreshUnmappedListens { state } => {
                let mut config_lock = CONFIG.write().await;

                config_lock.listens.config_refresh_unmapped_listens(*state);

                println_cli(format!(
                    "Successfully set `RefreshUnmappedListens` to {}",
                    config_lock.listens.refresh_unmapped_listens
                ));
            }
        }

        Ok(())
    }
}
