use color_eyre::eyre::Ok;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    return Ok(());
}
