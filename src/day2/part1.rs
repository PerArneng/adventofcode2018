
use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;
use itertools::Itertools;


#[derive(PartialEq)]
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
        .collect::<Vec<Repetition>>()
}