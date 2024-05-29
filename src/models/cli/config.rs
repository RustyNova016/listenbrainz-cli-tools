use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use derive_more::*;

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

    Edit {
        edited_mbid: String,

        on: SelfEditType,

        action: SelfEditActionValue,

        edit_target: Option<String>,
    }
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

#[derive(Debug, IsVariant, Unwrap, Clone, ValueEnum)]
pub enum SelfEditType {
    RadioSeeding,
    RadioInsert,
    StatCounting
}

#[derive(Debug, IsVariant, Unwrap, Clone, Default, ValueEnum)]
pub enum SelfEditActionValue {
    MergeInto,
    Abort,
    #[default]
    None
}