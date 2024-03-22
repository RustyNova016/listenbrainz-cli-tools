use clap::Parser;
use color_eyre::eyre::Ok;

use colored::*;
use models::cli::Cli;

pub mod models;
pub mod tools;
pub mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("Hello!");

    cli.command.run();

    Ok(())
}
