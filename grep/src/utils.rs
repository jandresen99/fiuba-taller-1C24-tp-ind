use std::error::Error;
use std::env;

/// READ_ARGS: lee los argumentos pasados por comando y devuelve una lista con ellos
pub fn read_args() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        Err("Missing arguments")?;
    }

    Ok(args)
}