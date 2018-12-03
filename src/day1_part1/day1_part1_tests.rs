
use day1_part1;

#[test]
fn parse_frequency_simple_happy_path() {
    let parse = day1_part1::parse_frequency;
    assert_eq!(parse("+1").unwrap(), 1);
    assert_eq!(parse("10").unwrap(), 10);
    assert_eq!(parse("-5").unwrap(), -5);
    assert_eq!(parse("0").unwrap(), 0);
}

#[test]
fn calculate_sum_of_frequencies_doc_example1() {
    let calc = day1_part1::calculate_sum_of_frequencies;
    let numbers = vec![1, 1, 1];
    assert_eq!(calc(&numbers), 3);
}

#[test]
fn calculate_sum_of_frequencies_doc_example2() {
    let calc = day1_part1::calculate_sum_of_frequencies;
    let numbers = vec![1, 1, -2];
    assert_eq!(calc(&numbers), 0);
}

#[test]
fn calculate_sum_of_frequencies_doc_example3() {
    let calc = day1_part1::calculate_sum_of_frequencies;
    let numbers = vec![-1, -2, -3];
    assert_eq!(calc(&numbers), -6);
}
