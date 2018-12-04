

use aoc_utils;
use std::fs::File;
use std::io::{self, BufRead, BufReader};



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


pub fn calculate_sum_of_frequencies(vector:&Vec<i32>) -> i32 {
    return vector.into_iter().fold(0, |a,b| a + b);
}


pub fn start(input:&str) -> io::Result<()> {
    println!(" :- Day 1 Part 1");
    println!("    using file: '{}'", input);

    aoc_utils::ensure_file(input);
    let numbers = read_frequencies(input)?;
    let total_freq = calculate_sum_of_frequencies(&numbers);

    println!("    result: {}", total_freq);

    Ok(())
}