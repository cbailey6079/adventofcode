use crate::{utils, Day};

pub struct Day7 {}

impl Day for Day7 {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day7/files/{file}.txt").as_str());


        "".to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day7/files/{file}.txt").as_str());

        "".to_string()
    }
}


#[cfg(test)]
#[test]
fn day7_part1_example() {
    let day = Day7 {};
    let expected = "";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day7_part1() {
    let day = Day7 {};
    let expected = "";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day7_part2_example() {
    let day = Day7 {};
    let expected = "";

    let actual = day.part2("test2".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day7_part2() {
    let day = Day7 {};
    let expected = "";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}