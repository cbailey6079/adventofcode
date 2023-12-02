#[cfg(test)]

use crate::days::day2;

#[test]
fn day2_part1_example() {
    let expected = "8";
    let actual = day2::run(1, "test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day2_part1() {
    let expected = "3035";
    let actual = day2::run(1, "input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day2_part2_example() {
    let expected = "2286";
    let actual = day2::run(2, "test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day2_part2() {
    let expected = "66027";
    let actual = day2::run(2, "input".to_string());

    assert_eq!(actual, expected);
}