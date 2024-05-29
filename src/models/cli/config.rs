use clap::Parser;
use clap::Subcommand;

use crate::models::config::Config;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ConfigCli {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    SetToken {
        /// Name of the user to add the token
        username: String,

        /// User token
        token: String,
    },
}

impl ConfigCommands {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::SetToken { username, token } => {
                let mut conf = Config::load()?;
                conf.set_token(username.clone(), token.clone());
                conf.save()?;
            }
        }

        Ok(())
    }
}
