pub mod remap;
use clap::Parser;
use clap::Subcommand;
use remap::Remapper;

use crate::models::config::Config;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ListenCommand {
    /// Name of the user to fetch unlinked listen from
    username: String,

    /// User token
    #[arg(short, long)]
    token: Option<String>,

    #[command(subcommand)]
    command: ListenSubCommands,
}

impl ListenCommand {
    pub fn get_token(&self) -> String {
        Config::get_token_or_argument(&self.username, &self.token)
    }

    pub async fn run(&self) -> color_eyre::Result<()> {
        self.command
            .run(self.username.clone(), self.get_token())
            .await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListenSubCommands {
    Remap {},
}

impl ListenSubCommands {
    pub async fn run(&self, username: String, token: String) -> color_eyre::Result<()> {
        match self {
            Self::Remap {} => {
                Remapper::run(username, token).await;
            }
        }

        Ok(())
    }
}
