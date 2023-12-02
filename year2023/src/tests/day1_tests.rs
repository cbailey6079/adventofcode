#[cfg(test)]

use crate::days::day1;

#[test]
fn day1_part1_example() {
    let expected = "142";
    let actual = day1::run(1, "test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day1_part1() {
    let expected = "54990";
    let actual = day1::run(1, "input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day1_part2_example() {
    let expected = "281";
    let actual = day1::run(2, "test2".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day1_part2() {
    let expected = "54473";
    let actual = day1::run(2, "input".to_string());

    assert_eq!(actual, expected);
}