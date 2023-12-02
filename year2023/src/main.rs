use std::io::stdin;

use crate::days::{day1, day2};

mod days;
mod utils;
mod tests;

fn main() {
    loop {
        println!("What day would you like to run?");

        let mut day_buffer = String::new();
        let mut part_buffer = String::new();

        let _ = stdin().read_line(&mut day_buffer);

        println!("What part for Day {} would you like to run?", day_buffer.trim());

        match stdin().read_line(&mut part_buffer) {
            Ok(_) => {
                let parse = part_buffer.trim_end().parse::<i8>();
                match parse {
                    Ok(part) => {
                        match day_buffer.trim_end() {
                            "1" => println!("answer: {}", day1::run(part, "input".to_string())),
                            "2" => println!("answer: {}", day2::run(part, "input".to_string())),
                            _ => continue
                        }
                    },
                    Err(_) => continue,
                }
            },
            Err(_) => continue,
        };
    }
}