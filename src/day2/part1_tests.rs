
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
    assert!(cr("aabcdd").contains(&Repetition::Double));
    assert!(cr("abcdee").contains(&Repetition::Double));
    assert!(cr("ababab").contains(&Repetition::Triplet));
}
