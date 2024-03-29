use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::{BufReader, Error, ErrorKind};

/// READ_ARGS: lee los argumentos pasados por comando y devuelve una lista con ellos
pub fn read_args() -> Result<Vec<String>, std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        Error::new(ErrorKind::Other, "Missing arguments");
    } else if args.len() > 3 {
        Error::new(ErrorKind::Other, "Too many arguments");
    }

    Ok(args)
}

/// READ_LINES: lee el archivo indicado y devuelve una lista con cada linea
pub fn read_lines(filename: String) -> Result<Vec<String>, std::io::Error> {
    let mut rows = Vec::new();
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        rows.push(line?);
    }

    Ok(rows)
}
