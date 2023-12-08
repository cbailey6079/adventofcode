use crate::{utils, Day};

pub struct Day8 {}

impl Day for Day8 {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day8/files/{file}.txt").as_str());


        "".to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day8/files/{file}.txt").as_str());

        "".to_string()
    }
}


#[cfg(test)]
#[test]
fn day8_part1_example() {
    let day = Day8 {};
    let expected = "";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day8_part1() {
    let day = Day8 {};
    let expected = "";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day8_part2_example() {
    let day = Day8 {};
    let expected = "";

    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day8_part2() {
    let day = Day8 {};
    let expected = "";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}