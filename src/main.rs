use clap::Parser;
use color_eyre::eyre::Ok;

use models::cli::Cli;

use crate::utils::println_cli;

pub mod core;
pub mod models;
/// This is the module containing all the different tools of this app
pub mod tools;
pub mod utils;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("Hello!");

    //println_cli("Cleaning some old entries...");
    //MUSICBRAINZ_DATABASE.invalidate_last_entries(10, 10).await?;
    //println_cli("Done!");

    cli.command
        .run()
        .await
        .expect("An error occured in the app");

    println_cli("Have a nice day!");
    Ok(())
}
