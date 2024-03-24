use std::error::Error;
use grep::utils::read_args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = read_args()?;

    println!("{:?}", args);

    Ok(())
}
