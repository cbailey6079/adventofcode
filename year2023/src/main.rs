use std::io::stdin;

use crate::utils::populate_days;

mod utils;
mod days;

fn main() {
    let days = populate_days();
    let mut day_number;
    let mut day;

    loop {
        let mut day_buffer = String::new();
        let mut part_buffer = String::new();

        println!("What day would you like to run?");

        let _ = stdin().read_line(&mut day_buffer);

        match day_buffer.trim_end().parse::<usize>() {
            Ok(number) =>  day_number = number,
            Err(_) => { println!("Please enter a number from 1 to 25. lol \n\n"); continue; },
        }

        match days.iter().nth(day_number - 1) {
            Some(d) => day = d,
            None => { println!("Day does not exist.....yet? \n\n"); continue; },
        }

        println!("What part for Day {} would you like to run?", day_number);

        match stdin().read_line(&mut part_buffer) {
            Ok(_) => {
                let parse = part_buffer.trim_end().parse::<i8>();
                match parse {
                    Ok(part) => {
                        let answer = match part {
                            1 => day.part1("input".to_string()),
                            2 => day.part2("input".to_string()),
                            _ => continue,
                        };

                        println!("answer: {} \n\n", answer);
                    },
                    Err(_) => continue,
                }
            },
            Err(_) => continue,
        };
    }
}

pub trait Day {
    fn part1(&self, file: String) -> String;
    fn part2(&self, file: String) -> String;
}