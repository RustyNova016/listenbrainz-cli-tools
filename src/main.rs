use color_eyre::eyre::Ok;

pub mod models;
/// This is the module containing all the different tools of this app
pub mod tools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    return Ok(());
}
