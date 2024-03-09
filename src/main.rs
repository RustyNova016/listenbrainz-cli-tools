use clap::Parser;
use color_eyre::eyre::Ok;

use models::cli::Cli;

pub mod models;
pub mod tools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    cli.command.run();

    Ok(())
}
