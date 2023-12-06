#[cfg(test)]

use crate::{days::day5::Day5, Day};

#[test]
fn day5_part1_example() {
    let day = Day5 {};
    let expected = "35";
    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day5_part1() {
    let day = Day5 {};
    let expected = "278755257";
    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day5_part2_example() {
    let day = Day5 {};
    let expected = "46";
    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

// you brute forced this problem because you are lazy, so don't run it.
// #[test]
// fn day5_part2() {
//     let day = Day5 {};
//     let expected = "26829166";
//     let actual = day.part2("input".to_string());

//     assert_eq!(actual, expected);
// }