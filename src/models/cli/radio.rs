use clap::ArgAction;
use clap::{Parser, Subcommand};

use crate::core::statistics::listen_rate::ListenRate;
use crate::core::statistics::listen_rate::ListenRateRange;
use crate::tools::radio::circles::create_radio_mix;
use crate::tools::radio::listen_rate::listen_rate_radio;
use crate::tools::radio::overdue::overdue_radio;
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
        ///// The amount of hours needed to wait after a recording have been given before it is resuggested
        //#[arg(short, long, default_value_t = 0)]
        //cooldown: u64
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

    /// Generate playlists depending on the listen rate of recordings
    Rate {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: String,

        /// Minimum listen rate
        #[arg(long)]
        min_rate: Option<u64>,

        /// Minimum listen rate time range
        #[arg(long)]
        min_per: Option<ListenRateRange>,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,
    },

    /// Generate playlists based on recording that the user should have listened to by now according to the user's listen rate
    Overdue {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: String,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,
    },
}

impl Radios {
    pub async fn run(&self) {
        match self {
            Self::Circles {
                username,
                token,
                unlistened,
                //cooldown
            } => create_radio_mix(username, token.clone(), *unlistened).await,

            Self::Underrated { username, token } => {
                underrated_mix(username.clone(), token.clone()).await;
            }

            Self::Rate {
                username,
                token,
                min_rate,
                min_per,
                min,
                cooldown,
            } => {
                let mut rate = None;

                if let Some(min_rate) = min_rate {
                    if let Some(min_per) = min_per {
                        rate = Some(ListenRate::new(
                            "*".to_string().into(),
                            *min_rate,
                            min_per.get_duration(),
                        ));
                    }
                }

                listen_rate_radio(username, token, rate, *min, *cooldown).await;
            }

            Self::Overdue {
                username,
                token,
                min,
                cooldown,
            } => {
                overdue_radio(username, token, *min, *cooldown).await;
            }
        }
    }
}
