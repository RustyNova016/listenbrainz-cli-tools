use crate::tools::{
    stats::{artist_stats, recording_stats},
    unlinked::unlinked_command,
};
use clap::{Parser, Subcommand};

/// Tools for Listenbrainz
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Tools with the unlinked listens
    Unlinked {
        /// Name of the user to fetch unlinked listen from
        #[arg(short, long)]
        username: String,
    },

    /// Live and accurate statistics
    Stats {
        #[command(subcommand)]
        command: StatsCommand,
    },
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Unlinked { username } => unlinked_command(username),
            Commands::Stats { command } => command.run(),
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum StatsCommand {
    /// Get recording stats (Default)
    Recordings {
        /// Name of the user to fetch stats listen from
        #[arg(short, long)]
        username: String,
    },

    /// Get artist stats.
    Artist {
        /// Name of the user to fetch stats listen from
        #[arg(short, long)]
        username: String,
    },
}

impl StatsCommand {
    pub fn run(&self) {
        match self {
            StatsCommand::Recordings { username } => recording_stats(username),
            StatsCommand::Artist { username } => artist_stats(username),
        }
    }
}
