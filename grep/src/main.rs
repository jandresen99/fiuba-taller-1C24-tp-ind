use grep::{
    regex::Regex,
    utils::{read_args, read_lines},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = read_args()?;

    let expression = &args[1];
    let value = &args[2];
    //let filepath = &args[2];

    //let lines = read_lines(filepath.to_string())?;

    let regex = Regex::new(expression);

    match regex.unwrap().test(value) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error {}", err),
    }

    Ok(())
}

// PENDIENTE: si no tengo un ^ adelante, le pondo un Any Wildcard al principio 1:27:50
