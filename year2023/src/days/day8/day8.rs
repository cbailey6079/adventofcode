use std::collections::HashMap;

use crate::{utils, Day};

pub struct Day8 {}

#[derive(Debug)]
struct MyTup {
    left: String,
    right: String,
}

impl Day for Day8 {
    fn part1(&self, file: String) -> String {
        let (map, nav) = parse_map(file);
        let mut steps = 1;
        let mut curr_step = "AAA";

        loop {
            let test = match nav[(steps - 1) % nav.len()].as_str() {
                "L" => &map[curr_step].left,
                _ => &map[curr_step].right,
            };

            if test == "ZZZ" {
                break;
            } else {
                curr_step = test;
            }

            steps += 1;
        } 

        steps.to_string()
    }

    fn part2(&self, file: String) -> String {
        let (map, nav) = parse_map(file);
        let mut steps: Vec<usize> = Vec::new();
        let curr_steps: Vec<String> = map.iter()
            .filter(|a| a.0.ends_with("A"))
            .map(|b| b.0.to_string())
            .collect();

        for (i, mut curr_step) in curr_steps.iter().enumerate() {
            steps.push(1);

            loop {
                let test = match nav[(steps[i] - 1) % nav.len()].as_str() {
                    "L" => &map[curr_step].left,
                    _ => &map[curr_step].right,
                };

                if test.contains("Z") {
                    break;
                } else {
                    curr_step = test;
                }

                steps[i] += 1;
            } 
        }

        lcm(steps).to_string()
    }
}

fn parse_map(file: String) -> (HashMap<String, MyTup>, Vec<String>) {
    let lines = utils::read_lines(format!("./src/days/day8/files/{file}.txt").as_str());
    let mut nav: Vec<String> = Vec::new();
    let mut map: HashMap<String, MyTup> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() { continue; }

        if i == 0 {
            nav = line.split("")
                .filter(|a| *a != "")
                .map(|b| b.to_string())
                .collect::<Vec<String>>();

            continue;
        }   

        let first_part: Vec<&str> = line.split(" = ").collect();
        let key = first_part[0].to_string();
        let second_part: Vec<&str> = first_part[1].split(", ").collect();
        
        let mut temp = second_part[0].to_string();
        temp.remove(0);
        let left = temp;

        temp = second_part[1].to_string();
        temp.pop();
        let right = temp;

        map.insert(key, MyTup { left, right });
    }

    (map, nav)
}

fn lcm(steps: Vec<usize>) -> usize {
    let mut lcm = steps[0];

    for i in 1..steps.len() {
        lcm = ((steps[i] * lcm) / (gcd(steps[i], lcm))).clone();
    }

    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[cfg(test)]
#[test]
fn day8_part1_example() {
    let day = Day8 {};
    let expected = "2";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day8_part1() {
    let day = Day8 {};
    let expected = "13301";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day8_part2_example() {
    let day = Day8 {};
    let expected = "6";

    let actual = day.part2("test2".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day8_part2() {
    let day = Day8 {};
    let expected = "7309459565207";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}