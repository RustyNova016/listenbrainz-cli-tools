use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use derive_more::*;

use crate::models::config::Config;
use crate::models::data::musicbrainz::mbid::mbid_kind::MBIDKind;
use crate::models::data::musicbrainz::mbid::MBID;

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

        on: SelfEditContext,

        action: SelfEditActionValue,

        edit_target: Option<String>,
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

            Self::Edit {
                edited_mbid,
                on,
                action,
                edit_target,
            } => {
                Config::set_edit(
                    MBID::from_string( edited_mbid, MBIDKind::Recording)?,
                    *on,
                    action.clone(),
                    edit_target.as_ref().map(|id| MBID::from_string( id, MBIDKind::Recording)).transpose()?,
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, IsVariant, Unwrap, Clone, ValueEnum, Copy)]
pub enum SelfEditContext {
    RadioSeeding,
    RadioInsert,
    StatCounting,
}

#[derive(Debug, IsVariant, Unwrap, Clone, Default, ValueEnum)]
pub enum SelfEditActionValue {
    MergeInto,
    Abort,
    #[default]
    None,
}
