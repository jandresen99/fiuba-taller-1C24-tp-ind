use std::error::Error;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


/// READ_ARGS: lee los argumentos pasados por comando y devuelve una lista con ellos
pub fn read_args() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        Err("Missing arguments")?;
    } else if  args.len() > 3 {
        Err("Too many arguments")?;
    }

    Ok(args)
}

/// READ_LINES: lee el archivo indicado y devuelve una lista con cada linea
pub fn read_lines(filename: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rows = Vec::new();
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    for line in reader.lines(){
        rows.push(line?);
    }

    Ok(rows)
}