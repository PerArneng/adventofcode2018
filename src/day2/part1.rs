
use std::collections::{HashMap,HashSet};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
pub enum Repetition {
    None,
    Double,
    Triplet,
    DoubleAndTriplet
}


pub fn calculate_repetition(id:&str) -> Repetition {

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


    let reps:HashSet<Repetition> = char_freq.values().map(
        |freq| match freq {
            1 => Repetition::None,
            2 => Repetition::Double,
            _ => Repetition::Triplet
        }
    ).filter(|rep| *rep != Repetition::None)
        .collect();

    //char_map.iter().for_each(|f| println!("{} {}", f.0, f.1));

    return Repetition::None;
}