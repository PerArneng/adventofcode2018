
use crate::day2::part1::{self, Repetition};

#[test]
fn calculate_repetition_test() {
    let cr = part1::calculate_repetition;
    assert_eq!(cr("abcdef").len(), 0);

    let rep = cr("bababc");
    assert_eq!(rep.len(), 2);
    assert!(rep.contains(&Repetition::Double) &&
            rep.contains(&Repetition::Triplet)
    );

    assert!(cr("abbcde").contains(&Repetition::Double));
    assert!(cr("abcccd").contains(&Repetition::Triplet));

    let rep = cr("aabcdd");
    assert_eq!(rep.len(), 1);
    assert!(rep.contains(&Repetition::Double));

    assert!(cr("abcdee").contains(&Repetition::Double));
    assert!(cr("ababab").contains(&Repetition::Triplet));
}

#[test]
fn calculate_checksum_test() {

    let ids = vec!( String::from("abcdef"),
                            String::from("bababc"),
                            String::from("abbcde"),
                            String::from("abcccd"),
                            String::from("aabcdd"),
                            String::from("abcdee"),
                            String::from("ababab"));

    assert_eq!(12, part1::calculate_checksum(&ids));
}