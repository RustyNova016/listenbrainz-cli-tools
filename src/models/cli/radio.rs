use clap::ArgAction;
use clap::{Parser, Subcommand};

use crate::tools::radio::circles::create_radio_mix;
use crate::tools::radio::underrated::underrated_mix;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CliRadios {
    #[command(subcommand)]
    pub command: Radios,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Radios {
    /// Radio based on artist that already got listened to
    Circles {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: String,

        /// Use this flag to only get unlistened recordings
        #[clap(long, action=ArgAction::SetTrue)]
        unlistened: bool,
    },

    /// Generate a playlist containing your underrated listens
    Underrated {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: String,
    },
}

impl Radios {
    pub async fn run(&self) {
        match self {
            Self::Circles {
                username,
                token,
                unlistened,
            } => create_radio_mix(username, token.clone(), *unlistened).await,
            Self::Underrated { username, token } => {
                underrated_mix(username.clone(), token.clone()).await;
            }
        }
    }
}
