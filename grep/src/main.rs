use std::error::Error;
use grep::utils::{read_args, read_lines};

fn main() -> Result<(), Box<dyn Error>> {
    let args = read_args()?;
    println!("{:?}", args);

    let filepath = &args[2];

    let lines = read_lines(filepath.to_string())?;
    println!("{:?}", lines);

    Ok(())
}
