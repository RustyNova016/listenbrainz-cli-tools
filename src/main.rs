use clap::{Command, Parser};
use color_eyre::eyre::Ok;

use crate::{models::{Cli, Commands}, tools::get_all_unlinked_of_user};

pub mod models;
/// This is the module containing all the different tools of this app
pub mod tools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("Hello, world!");

    match &cli.command {
        Commands::Unlinked { username } => {
            println!("Fetching unlinkeds for user {}", username);
            let unlinked = get_all_unlinked_of_user(username);
            for ele in unlinked {
                println!("{:#?} - {} [{}]", ele.track_metadata.release_name, ele.track_metadata.artist_name, ele.recording_msid)
            }
        }
    }

    Ok(())
}
