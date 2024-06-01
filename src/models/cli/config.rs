use std::fmt::{Display, Formatter};

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use derive_more::*;
use inquire::{Select, Text};

use crate::models::config::Config;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::mbid::mbid_kind::MBIDKind;
use crate::utils::println_cli;

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

    EditInteractive,
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
                    MBID::from_string(edited_mbid, MBIDKind::Recording)?,
                    *on,
                    action.clone(),
                    edit_target
                        .as_ref()
                        .map(|id| MBID::from_string(id, MBIDKind::Recording))
                        .transpose()?,
                );
            }
            Self::EditInteractive => interactive_edit(),
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

impl Display for SelfEditContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SelfEditContext::RadioSeeding => {
                write!(f, "Radio Seeding")
            }
            SelfEditContext::RadioInsert => {
                write!(f, "Radio Inserting")
            }
            SelfEditContext::StatCounting => {
                write!(f, "Statistic counting")
            }
        }
    }
}

#[derive(Debug, IsVariant, Unwrap, Clone, Default, ValueEnum)]
pub enum SelfEditActionValue {
    MergeInto,
    Abort,
    #[default]
    None,
}

impl Display for SelfEditActionValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SelfEditActionValue::MergeInto => {
                write!(f, "Merge into")
            }
            SelfEditActionValue::Abort => {
                write!(f, "Abort")
            }
            SelfEditActionValue::None => {
                write!(f, "None")
            }
        }
    }
}

pub fn interactive_edit() {
    let edited_mbid = prompt_edited_mbid();
    let context = prompt_context().expect("Couldn't get the context value");
    let action = prompt_action().expect("Couldn't get the action value");
    let mut target = None;

    if action.is_merge_into() {
        target = Some(prompt_merged_mbid());
    }

    Config::set_edit(edited_mbid, context, action, target);
    println_cli("All set!");
}

fn prompt_edited_mbid() -> MBID {
    loop {
        let Ok(edited_mbid_res) = Text::new("What MBID do you want to edit?").prompt() else {
            println_cli("Couldn't get the response. Please try again");
            continue;
        };

        let Ok(edited_mbid_parse) = MBID::from_string(&edited_mbid_res, MBIDKind::Recording) else {
            println_cli("Couldn't parse the response for any MBID. If you are sure that there is one, but see this error, please send a ticket.");
            continue;
        };

        return edited_mbid_parse;
    }
}

fn prompt_merged_mbid() -> MBID {
    loop {
        let Ok(edited_mbid_res) = Text::new("What MBID do you want to merge into?").prompt() else {
            println_cli("Couldn't get the response. Please try again");
            continue;
        };

        let Ok(edited_mbid_parse) = MBID::from_string(&edited_mbid_res, MBIDKind::Recording) else {
            println_cli("Couldn't parse the response for any MBID. If you are sure that there is one, but see this error, please send a ticket.");
            continue;
        };

        return edited_mbid_parse;
    }
}

fn prompt_context() -> color_eyre::Result<SelfEditContext> {
    let context = vec![SelfEditContext::RadioSeeding, SelfEditContext::RadioInsert];

    Ok(Select::new("In what context this edit apply?", context).prompt()?)
}

fn prompt_action() -> color_eyre::Result<SelfEditActionValue> {
    let actions = vec![
        SelfEditActionValue::None,
        SelfEditActionValue::Abort,
        SelfEditActionValue::MergeInto,
    ];

    Ok(Select::new("What should it do?", actions).prompt()?)
}
