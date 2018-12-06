

use std::io::{self, BufRead, BufReader};
use std::fs::File;


pub fn parse_frequency(frequency_string:&str) -> io::Result<i32> {

    let frequency_result = frequency_string.parse::<i32>();
    match frequency_result {
        Ok(freq) => Ok(freq),
        Err(err) => Err(
            io::Error::new(io::ErrorKind::InvalidData, err.to_string())
        )
    }
}


pub fn read_frequencies(input:&str) -> io::Result<Vec<i32>> {

    let f = File::open(input)?;
    let reader = BufReader::new(f);

    let mut numbers = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let frequency = parse_frequency(&line)?;
        numbers.push(frequency);
    }

    Ok(numbers)
}