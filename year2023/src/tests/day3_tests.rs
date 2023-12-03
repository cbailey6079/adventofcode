#[cfg(test)]

use crate::{days::day3::Day3, Day};

#[test]
fn day3_part1_example() {
    let day = Day3 {};
    let expected = "4361";
    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day3_part1() {
    let day = Day3 {};
    let expected = "536202";
    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day3_part2_example() {
    let day = Day3 {};
    let expected = "467835";
    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day3_part2() {
    let day = Day3 {};
    let expected = "78272573";
    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}