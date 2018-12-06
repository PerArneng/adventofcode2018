
use crate::day1::part2;


#[test]
fn find_first_duplicate_frequency_test1() {
    let data = vec![1, -1];
    assert_eq!(part2::find_first_duplicate_frequency(&data), Some(0));
}


#[test]
fn find_first_duplicate_frequency_test2() {
    let data = vec![3, 3, 4, -2, -4];
    assert_eq!(part2::find_first_duplicate_frequency(&data), Some(10));
}


#[test]
fn find_first_duplicate_frequency_test3() {
    let data = vec![-6, 3, 8, 5, -6];
    assert_eq!(part2::find_first_duplicate_frequency(&data), Some(5));
}


#[test]
fn find_first_duplicate_frequency_test4() {
    let data = vec![7, 7, -2, -7, -4];
    assert_eq!(part2::find_first_duplicate_frequency(&data), Some(14));
}