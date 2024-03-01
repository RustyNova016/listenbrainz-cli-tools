use color_eyre::eyre::Ok;

/// This is the module containing all the different tools of this app
pub mod tools;
pub mod models;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    return Ok(());
}
