use clap::Parser;
use color_eyre::eyre::Ok;

use models::cli::Cli;

use crate::models::cache::global_cache_save;
use crate::utils::println_cli;

pub mod models;
pub mod tools;
pub mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("Hello!");

    cli.command.run();

    println_cli("Saving cache...");
    global_cache_save()?;
    println_cli("Have a nice day!");
    Ok(())
}
