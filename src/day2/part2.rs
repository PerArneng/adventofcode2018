#[cfg(test)]
pub mod tests;

use std::io;
use crate::aoc_utils;

fn get_similar_chars(str1:&str, str2:&str) -> String {
        str1.chars()
            .zip(str2.chars())
            .filter(|pair| pair.0 == pair.1)
            .map(|pair| pair.0)
            .collect::<String>()
}


fn find_similar_ids(ids:&Vec<String>) -> Vec<String> {

    let mut found:Vec<String> = Vec::new();

    for id in ids {
        for id2 in ids {
            let similar = get_similar_chars(id, id2);
            if similar.len() == (id.len() - 1) {
                found.push(similar);
            }
        }
    }


    return found;
}

pub fn start(input:&str) -> io::Result<()> {
    println!(" :- Day 2 Part 2");
    println!("    using file: '{}'", input);

    aoc_utils::ensure_file(input);
    let lines = aoc_utils::read_lines(input)?;
    let similar = find_similar_ids(&lines);

    println!("    result: {}", similar.get(0).unwrap());

    Ok(())
}