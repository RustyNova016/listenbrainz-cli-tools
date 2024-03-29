use clap::{Parser, Subcommand};

use listenbrainz::raw::response::UserListensListen;

use crate::tools::unlinked::unlinked_command;

pub mod messy_recording;

pub struct UnlinkedListenCollection(Vec<UserListensListen>);

impl UnlinkedListenCollection {
    pub fn push(&mut self, item: UserListensListen) {
        if item.track_metadata.mbid_mapping.is_none() {
            self.0.push(item)
        }
    }
}

impl Extend<UserListensListen> for UnlinkedListenCollection {
    fn extend<T: IntoIterator<Item = UserListensListen>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

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
