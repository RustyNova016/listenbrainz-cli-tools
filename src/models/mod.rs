use clap::{Parser, Subcommand};
use listenbrainz::raw::response::UserListensListen;

pub struct UnlinkedListenCollection(Vec<UserListensListen>);

impl UnlinkedListenCollection {
    pub fn push(&mut self, item: UserListensListen) {
        if item.track_metadata.mbid_mapping.is_none() {
            self.0.push(item)
        }
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
