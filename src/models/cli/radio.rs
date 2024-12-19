use std::ops::Deref;

use clap::ArgAction;
use clap::{Parser, Subcommand};

use crate::datastructures::radio::collector::RadioCollector;
use crate::datastructures::radio::collector::RadioCollectorBuilder;
use crate::datastructures::radio::seeders::listens::ListenSeeder;
use crate::datastructures::radio::seeders::listens::ListenSeederBuilder;
use crate::datastructures::radio::seeders::SeederSettings;
use crate::datastructures::radio::seeders::SeederSettingsBuilder;
use crate::models::config::Config;
use crate::tools::radio::circles::create_radio_mix;
use crate::tools::radio::listen_rate::listen_rate_radio;
use crate::tools::radio::overdue::overdue_radio;
use crate::tools::radio::underrated::underrated_mix;
//use crate::tools::radio::underrated::underrated_mix;

use super::common::Timeframe;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RadioCommand {
    #[command(subcommand)]
    pub command: RadioSubcommands,

    /// The minimum count of tracks the radio should add to the playlist. (Default: 50, gets overidden by `--min-duration`)
    #[arg(long)]
    min_count: Option<u64>,

    /// The minimum duration the playlist should last for. This accept natural language (Ex: "1 hour 36 mins")
    #[arg(long)]
    min_duration: Option<String>,

    /// For radios based on listens, what time range of listens to use as reference
    #[arg(long)]
    seed_listen_range: Option<Timeframe>,

    /// When used with `seed_listen_range`, how many listens should be given as a minimum, even if they are outside of the range (Default: 3)
    #[arg(long)]
    min_seed_listens: Option<u64>,
}

impl RadioCommand {
    pub fn get_collector(&self) -> RadioCollector {
        let collector = RadioCollectorBuilder::default();

        let collector = match self.min_count {
            Some(val) => collector.count(val),
            None => collector.count_none(),
        };

        let collector = match self.min_duration.as_ref() {
            Some(val) => {
                let dura: humantime::Duration = val
                    .clone()
                    .parse()
                    .expect("Couldn't parse mimimum lenght for radio");
                let std_dura = dura.deref();
                let chrono_dura = chrono::Duration::from_std(*std_dura).unwrap();
                collector.duration(chrono_dura)
            }
            None => collector.duration_none(),
        };

        collector.build()
    }

    fn get_seeder_settings(&self) -> SeederSettings {
        SeederSettingsBuilder::default()
            .min_listen_per_recording(self.min_seed_listens.unwrap_or(3))
            .min_listened_at(self.seed_listen_range.map(|r| r.get_start_date()))
            .max_listened_at_default()
            .build()
    }

    fn get_listen_seeder(&self, username: &Option<String>) -> ListenSeeder {
        ListenSeederBuilder::default()
            .username(Config::check_username(username))
            .settings(self.get_seeder_settings())
            .build()
    }

    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> color_eyre::Result<()> {
        self.command.run(conn, self.get_collector(), self).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum RadioSubcommands {
    /// Randomly adds recordings from artists you already listened to
    Circles {
        /// Name of the user to fetch listens from
        username: Option<String>,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        token: Option<String>,

        /// Use this flag to only get unlistened recordings. This is great for exploration playlists
        #[clap(long, action=ArgAction::SetTrue)]
        unlistened: bool,
    },

    /// Generate a playlist containing your underrated listens
    ///
    /// This radio will create a playlist containing all the tracks that you listen to, but seemingly no one else does.
    ///
    ///> The mix is made by calculating a score for each listen. This score is composed of two values:
    ///> - The rank in the user's top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)
    ///> - The percentage of the recording's listens being from the user (Made with this formula: (user listens / worldwide listens) *100)
    Underrated {
        /// Name of the user to fetch listens from
        username: Option<String>,
        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Generate playlists depending on the listen rate of recordings
    ///
    /// This algorythm bases itself on your listen rate of recording to get more forgotten tracks.
    /// It takes the recordings with the lowest listen rates, and put them into a playlist
    Rate {
        /// Name of the user to fetch listens from
        username: Option<String>,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,
    },

    /// Generate playlists based on recording that the user should have listened to by now
    ///
    /// Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen.
    /// It then put together a playlist made out of recordings you should have listened by now.
    Overdue {
        /// Name of the user to fetch listens from
        username: Option<String>,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,

        /// Sort the recordings by the time overdue / the average time between listens
        ///
        /// Instead of sorting by date, the listens are sorted by how many estimated listens should have happened by now (Time elapsed since last listen / Average time per listens)
        #[arg(short, long, default_value_t = false)]
        overdue_factor: bool,

        /// Makes `overdue_factor` more accurate by calculating the score at the time the listen will be listened at instead of now.
        #[arg(short, long, default_value_t = false)]
        at_listening_time:bool
    },
}

impl RadioSubcommands {
    pub async fn run(
        &self,
        conn: &mut sqlx::SqliteConnection,
        collector: RadioCollector,
        command: &RadioCommand,
    ) -> color_eyre::Result<()> {
        match self {
            Self::Circles {
                username,
                token,
                unlistened,
                //cooldown
            } => {
                create_radio_mix(
                    command.get_listen_seeder(username),
                    Config::check_token(&Config::check_username(username), token),
                    *unlistened,
                    collector,
                )
                .await;
            }

            Self::Underrated { username, token } => {
                underrated_mix(
                    conn,
                    command.get_listen_seeder(username),
                    collector,
                    &Config::check_token(&Config::check_username(username), token),
                )
                .await?;
            }
            Self::Rate {
                username,
                token,
                min,
                cooldown,
            } => {
                listen_rate_radio(
                    conn,
                    command.get_listen_seeder(username),
                    &Config::check_token(&Config::check_username(username), token),
                    *min,
                    *cooldown,
                    collector,
                )
                .await?;
            }

            Self::Overdue {
                username,
                token,
                min,
                cooldown,
                overdue_factor: delay_factor,
                at_listening_time
            } => {
                overdue_radio(
                    conn,
                    command.get_listen_seeder(username),
                    &Config::check_token(&Config::check_username(username), token),
                    *min,
                    *cooldown,
                    *delay_factor,
                    command.get_collector(),
                    *at_listening_time
                )
                .await?;
            }
        }

        Ok(())
    }
}
