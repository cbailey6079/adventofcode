use std::collections::HashMap;
use crate::{utils, Day};

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day1/files/{file}.txt").as_str());

        let mut numbers: Vec<i8>;
        let mut total:i64 = 0;

        for line in lines {
            numbers = Vec::new();

            for ch in line.chars().into_iter() {
                match ch.to_digit(10) {
                    Some(val) => numbers.push(val as i8),
                    None => continue, 
                }  
            }

            total += ((numbers[0] * 10) + numbers[numbers.len() - 1]) as i64;
        }

        total.to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day1/files/{file}.txt").as_str());
        
        let english_number = ["one","two","three","four","five","six","seven","eight","nine"];
        let mut numbers =  HashMap::new();
        let mut total:i64 = 0;

        for line in lines {

            for (i,ch) in line.chars().into_iter().enumerate() {
                match ch.to_digit(10) {
                    Some(val) => { numbers.insert(i, val as i8); },
                    None => continue,
                }
            }
            
            for (i, item) in english_number.into_iter().enumerate() {
                match line.find(item) {
                    Some(val) => { numbers.insert(val, (i + 1) as i8); },
                    None => continue,
                }
                match line.rfind(item) {
                    Some(val) => { numbers.insert(val, (i + 1) as i8); },
                    None => continue,
                }
            }

            let min = numbers.keys().min().unwrap();
            let max = numbers.keys().max().unwrap();

            total += ((numbers[min] * 10) + numbers[max]) as i64;
            
            numbers = HashMap::new();
        }

        total.to_string()
    }
}


#[cfg(test)]
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