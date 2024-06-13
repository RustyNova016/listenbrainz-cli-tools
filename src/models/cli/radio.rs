use std::ops::Deref;

use clap::ArgAction;
use clap::{Parser, Subcommand};

use crate::core::statistics::listen_rate::ListenRate;
use crate::core::statistics::listen_rate::ListenRateRange;
use crate::models::config::Config;
use crate::models::radio::RadioConfig;
use crate::models::radio::RadioConfigBuilder;
use crate::tools::radio::circles::create_radio_mix;
use crate::tools::radio::listen_rate::listen_rate_radio;
use crate::tools::radio::overdue::overdue_radio;
use crate::tools::radio::underrated::underrated_mix;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RadioCommand {
    #[command(subcommand)]
    pub command: RadioSubcommands,

    #[arg(long)]
    min_count: Option<u64>,

    #[arg(long)]
    min_duration: Option<String>,
}

impl RadioCommand {
    pub fn get_config(&self) -> RadioConfig {
        let mut config_builder = RadioConfigBuilder::default();

        if let Some(val) = self.min_count {
            config_builder.min_count(val);
        }

        if let Some(val) = self.min_duration.as_ref() {
            let dura: humantime::Duration = val
                .clone()
                .parse()
                .expect("Couldn't parse mimimum lenght for radio");
            let std_dura = dura.deref();
            let chrono_dura = chrono::Duration::from_std(*std_dura).unwrap();
            config_builder.min_duration(chrono_dura);
        }

        config_builder.build().expect("Couldn't generate config")
    }

    pub async fn run(&self) -> color_eyre::Result<()> {
        let config = self.get_config();

        self.command.run(config).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum RadioSubcommands {
    /// Radio based on artist that already got listened to
    Circles {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: Option<String>,

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
        token: Option<String>,
    },

    /// Generate playlists depending on the listen rate of recordings
    Rate {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,

        /// User token
        #[arg(short, long)]
        token: Option<String>,

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
        token: Option<String>,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,

        /// Sort the recordings by the time overdue / the average time between listens
        #[arg(short, long, default_value_t = false)]
        overdue_factor: bool,
    },
}

impl RadioSubcommands {
    pub async fn run(&self, config: RadioConfig) -> color_eyre::Result<()> {
        match self {
            Self::Circles {
                username,
                token,
                unlistened,
                //cooldown
            } => {
                create_radio_mix(
                    username,
                    Config::get_token_or_argument(username, token),
                    *unlistened,
                    config,
                )
                .await;
            }

            Self::Underrated { username, token } => {
                underrated_mix(
                    username.clone(),
                    Config::get_token_or_argument(username, token),
                    config,
                )
                .await?;
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

                listen_rate_radio(
                    username,
                    &Config::get_token_or_argument(username, token),
                    rate,
                    *min,
                    *cooldown,
                    config,
                )
                .await?;
            }

            Self::Overdue {
                username,
                token,
                min,
                cooldown,
                overdue_factor: delay_factor,
            } => {
                overdue_radio(
                    username,
                    &Config::get_token_or_argument(username, token),
                    *min,
                    *cooldown,
                    *delay_factor,
                    config,
                )
                .await?;
            }
        }

        Ok(())
    }
}
