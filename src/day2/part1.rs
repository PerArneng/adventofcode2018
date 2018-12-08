
#[cfg(test)]
pub mod tests;


use std::collections::{HashMap};
use std::io;
use crate::aoc_utils;

use itertools::Itertools;


#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Debug)]
pub enum Repetition {
    Double,
    Triplet
}


pub fn calculate_repetition(id:&str) -> Vec<Repetition> {

    let mut char_freq:HashMap<char, i32> = HashMap::new();

    id.chars().for_each(
        |chr|
                if char_freq.contains_key(&chr) {
                    let current = *(char_freq.get(&chr).unwrap());
                    char_freq.insert(chr, current + 1);
                } else {
                    char_freq.insert(chr, 1);
                }
    );

    char_freq
        .values()
        .map(|x| *x)
        .filter(|x| (*x) == 2 || (*x) == 3)
        .map(|n| if n == 2 { Repetition::Double } else { Repetition::Triplet })
        .unique()
        .collect::<Vec<Repetition>>()
}


pub fn calculate_checksum(ids:&Vec<String>) -> i32 {

    let mut doubles = 0;
    let mut tripples = 0;

    fn count_rep(reps:&Vec<Repetition>, rep:&Repetition) -> i32 {
        reps.iter()
            .filter(|x| (**x) == *rep)
            .count() as i32
    }

    for id in ids {
        let reps = calculate_repetition(id);
        doubles += count_rep(&reps, &Repetition::Double);
        tripples += count_rep(&reps, &Repetition::Triplet);
    }

    return doubles * tripples;
}


pub fn start(input:&str) -> io::Result<()> {
    println!(" :- Day 2 Part 1");
    println!("    using file: '{}'", input);

    aoc_utils::ensure_file(input);
    let lines = aoc_utils::read_lines(input)?;
    let checksum = calculate_checksum(&lines);

    println!("    result: {}", checksum);

    Ok(())
}