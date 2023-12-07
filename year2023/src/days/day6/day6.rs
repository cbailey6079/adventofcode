use crate::{utils, Day};

pub struct Day6 {}

impl Day for Day6 {
    fn part1(&self, file: String) -> String {
        let mut times_can_win: u16;
        let mut margin_of_error: u32 = 1;
        let lines = utils::read_lines(format!("./src/days/day6/files/{file}.txt").as_str());
        
        let times: Vec<u16> = lines[0]
            .split(":")
            .collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .map(|i| i.parse::<u16>().unwrap())
            .collect();

        let distances: Vec<u16> = lines[1]
            .split(":")
            .collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .map(|i| i.parse::<u16>().unwrap())
            .collect();

        for (i ,time) in times.iter().enumerate() {
            times_can_win = 0;
            
            for x in 0u16..*time {
                let my_distance = x * (time - x);

                if my_distance > distances[i] {
                    times_can_win += 1;
                }
            }
            margin_of_error *= times_can_win as u32;
        }

        margin_of_error.to_string()
    }

    fn part2(&self, file: String) -> String {
        let mut winning_range: Vec<u64> = Vec::new();
        let lines = utils::read_lines(format!("./src/days/day6/files/{file}.txt").as_str());
        
        let time = lines[0]
            .split(":")
            .collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .expect("number");

        let distance = lines[1]
            .split(":")
            .collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("").parse::<u64>()
            .expect("number");

        for x in 1u64..time {
            let my_distance = x * (time - x);

            if my_distance > distance {
                winning_range.push(x);
                break;
            }
        }

        for i in (1u64..time).rev() {
            let my_distance = i * (time - i);

            if my_distance > distance {
                winning_range.push(i);
                break;
            }
        }

        (winning_range[1] - winning_range[0] + 1).to_string()
    }
}

#[cfg(test)]
#[test]
fn day6_part1_example() {
    let day = Day6 {};
    let expected = "288";
    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day6_part1() {
    let day = Day6 {};
    let expected = "128700";
    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day6_part2_example() {
    let day = Day6 {};
    let expected = "71503";
    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day6_part2() {
    let day = Day6 {};
    let expected = "39594072";
    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}