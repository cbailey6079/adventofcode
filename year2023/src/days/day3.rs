use crate::{utils, Day};

pub struct Day3 {}

impl Day for Day3 {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/files/day3/{file}.txt").as_str());


        "part1".to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/files/day3/{file}.txt").as_str());


        "part2".to_string()
    }
}