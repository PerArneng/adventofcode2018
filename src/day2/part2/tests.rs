

use crate::day2::part2;

#[test]
fn get_similar_chars_test() {
    assert_eq!("fgij", part2::get_similar_chars(
        &String::from("fghij"), &String::from("fguij"))
    );
}

#[test]
fn get_similar_chars_test_strange_strings() {
    assert_eq!("", part2::get_similar_chars(
        &String::from(""), &String::from("x_1"))
    );
}


#[test]
fn find_similar_ids_test() {

    let ids:Vec<String> = vec!(
        String::from("abcde"),
        String::from("fghij"),
        String::from("klmno"),
        String::from("pqrst"),
        String::from("fguij"),
        String::from("axcye"),
        String::from("wvxyz")
    );

    let found_ids:Vec<String> = part2::find_similar_ids(&ids);

    assert_eq!(2, found_ids.len());
    assert_eq!("fgij", found_ids.get(0).unwrap());
}