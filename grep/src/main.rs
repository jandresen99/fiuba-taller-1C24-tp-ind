use grep::{
    regex::Regex,
    utils::{read_args, read_lines},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = read_args()?;

    let expression = &args[1];
    let filepath = &args[2];

    let lines = read_lines(filepath.to_string())?;

    for value in lines {
        let regex = Regex::new(expression);
        match regex?.test(&value) {
            Ok(result) => {
                if result {
                    println!("{}", &value)
                }
            }
            Err(err) => println!("Error {}", err),
        }
    }

    Ok(())
}