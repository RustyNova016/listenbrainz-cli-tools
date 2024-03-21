use crate::tools::unlinked::unlinked_command;
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
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Unlinked { username } => unlinked_command(username),
        }
    }
}
