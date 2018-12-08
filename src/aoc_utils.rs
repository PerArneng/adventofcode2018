
use std::path::{Path};
use std::process;
use std::io::{self, BufReader, BufRead};
use std::fs::{File};

pub fn ensure_file(path:&str) {
    if !Path::new(path).exists() {
        eprintln!("err: file does not exist! bailing out! : '{}'", path);
        process::exit(1);
    }
}


pub fn read_lines(input:&str) -> io::Result<Vec<String>> {

    let f = File::open(input)?;
    let reader = BufReader::new(f);

    let mut lines = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
    }

    Ok(lines)
}