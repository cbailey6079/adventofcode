#[cfg(test)]

use crate::{days::day1::Day1, Day};


#[test]
fn day1_part1_example() {
    let day = Day1 {};
    let expected = "142";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day1_part1() {
    let day = Day1 {};
    let expected = "54990";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day1_part2_example() {
    let day = Day1 {};
    let expected = "281";

    let actual = day.part2("test2".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day1_part2() {
    let day = Day1 {};
    let expected = "54473";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}