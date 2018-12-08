
#[cfg(test)]
pub mod tests;

use crate::aoc_utils;
use std::io;
use crate::day1::utils;


pub fn calculate_sum_of_frequencies(vector:&Vec<i32>) -> i32 {
    return vector.into_iter().fold(0, |a,b| a + b);
}


pub fn start(input:&str) -> io::Result<()> {
    println!(" :- Day 1 Part 1");
    println!("    using file: '{}'", input);

    aoc_utils::ensure_file(input);
    let numbers = utils::read_frequencies(input)?;
    let total_freq = calculate_sum_of_frequencies(&numbers);

    println!("    result: {}", total_freq);

    Ok(())
}