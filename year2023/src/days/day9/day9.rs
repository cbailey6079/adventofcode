use crate::{utils, Day};

pub struct Day9 {}

impl Day for Day9 {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day9/files/{file}.txt").as_str());
        let mut sequences: Vec<Vec<i64>> = Vec::new();
        let mut patterns: Vec<Vec<i64>> = Vec::new();

        for line in &lines {
            sequences.push(line.split(" ").map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        }

        for sequence in sequences {
            
        }

        "".to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day9/files/{file}.txt").as_str());

        "".to_string()
    }
}

fn get_diffs(curr_diff: Vec<i64>) {
    for i in 0..curr_diff.len() - 1 {

    }
}


#[cfg(test)]
#[test]
fn day9_part1_example() {
    let day = Day9 {};
    let expected = "";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day9_part1() {
    let day = Day9 {};
    let expected = "";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day9_part2_example() {
    let day = Day9 {};
    let expected = "";

    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day9_part2() {
    let day = Day9 {};
    let expected = "";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}