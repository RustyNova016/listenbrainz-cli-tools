use clap::Parser;
use color_eyre::eyre::Ok;

use crate::models::Cli;

pub mod models;
pub mod tools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("Hello, world!");

    cli.command.run();

    Ok(())
}
