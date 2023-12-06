use std::fs;

use crate::{Day, days};

pub fn read_lines(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    
    contents.lines().map(|x| x.trim_end().to_string()).collect()
}

pub fn populate_days() -> Vec<Box<dyn Day>> {
    let mut days: Vec<Box<dyn Day>> = Vec::new();
    
    days.push(Box::new(days::day1::Day1 {}));
    days.push(Box::new(days::day2::Day2 {}));
    days.push(Box::new(days::day3::Day3 {}));
    days.push(Box::new(days::day4::Day4 {}));
    days.push(Box::new(days::day5::Day5 {}));
    days.push(Box::new(days::day6::Day6 {}));

    days
}