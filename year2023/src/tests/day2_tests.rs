#[cfg(test)]

use crate::{days::day2::Day2, Day};

#[test]
fn day2_part1_example() {
    let day = Day2 {};
    let expected = "8";
    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day2_part1() {
    let day = Day2 {};
    let expected = "3035";
    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day2_part2_example() {
    let day = Day2 {};
    let expected = "2286";
    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day2_part2() {
    let day = Day2 {};
    let expected = "66027";
    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}