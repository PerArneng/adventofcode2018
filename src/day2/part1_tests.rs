
use day2::part1::{self, Repetition};

#[test]
fn calculate_repetition_test() {
    let cr = part1::calculate_repetition;
    //assert_eq!(cr("abcdef"), Repetition::None);
    assert_eq!(cr("bababc"), Repetition::DoubleAndTriplet);
    //assert_eq!(cr("abbcde"), Repetition::Double);
    //assert_eq!(cr("abcccd"), Repetition::Triplet);
    //assert_eq!(cr("aabcdd"), Repetition::Double);
    //assert_eq!(cr("abcdee"), Repetition::Double);
    //assert_eq!(cr("ababab"), Repetition::Triplet);
}
