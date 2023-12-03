use crate::{utils, Day};

pub struct Day3 {}

impl Day for Day3 {
    fn part1(&self, file: String) -> String {
        let engine = create_engine(file);
        let mut sum: Vec<u32> = Vec::new();
        let mut adjacent_numbers;

        for (y, row) in engine.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col.parse::<u32>().is_err() && col != "." {
                    adjacent_numbers = check_adjacent(&engine, (y, x));

                    sum.append(&mut adjacent_numbers);
                }
            }
        } 

        sum.iter().sum::<u32>().to_string()
    }

    fn part2(&self, file: String) -> String {
        let engine = create_engine(file);
        let mut sum: Vec<u32> = Vec::new();
        let mut adjacent_numbers;

        for (y, row) in engine.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col == "*" {
                    adjacent_numbers = check_adjacent(&engine, (y, x));

                    if adjacent_numbers.len() == 2 {
                        sum.push(adjacent_numbers.iter().product());
                    }
                }
            }
        } 

        sum.iter().sum::<u32>().to_string()
    }
}

fn create_engine(file: String) -> Vec<Vec<String>> {
    let mut engine: Vec<Vec<String>> =  Vec::new();
    let mut temp: Vec<String>;
    let mut number_str;
    let mut adding;
    let lines = utils::read_lines(format!("./src/files/day3/{file}.txt").as_str());

    for line in lines {
        number_str = String::new();
        adding = false;
        temp = Vec::new();

        for ch in line.chars() {
            if ch.is_digit(10) {
                number_str = number_str.to_owned() + &ch.to_string();
                adding = true;
            } else {
                if adding {
                    for _ in number_str.chars() {
                        temp.push(number_str.clone());
                    }
                    number_str = String::new();
                }
                temp.push(ch.to_string());
                adding = false;
            }
        }

        if adding {
            for _ in number_str.chars() {
                temp.push(number_str.clone());
            }
        }

        engine.push(temp.clone());
    }

    engine
}

fn check_adjacent(engine: &Vec<Vec<String>>, location: (usize,usize)) -> Vec<u32> {
    let (y, x) = location;
    let row = &engine[y];
    let mut adjecent: Vec<u32> = Vec::new();

    // check top
    if y > 0 {
        if engine[y - 1][x].parse::<u32>().is_ok() { 
            adjecent.push(engine[y - 1][x].parse::<u32>().unwrap()); 
        } else {

            if x > 0 {
                if engine[y - 1][x - 1].parse::<u32>().is_ok() { 
                    adjecent.push(engine[y - 1][x - 1].parse::<u32>().unwrap()); 
                }
            }

            if x < row.len() - 1 {
                if engine[y - 1][x + 1].parse::<u32>().is_ok() { 
                    adjecent.push(engine[y - 1][x + 1].parse::<u32>().unwrap()); 
                }
            }
        }
    }

    // check bottom
    if y < engine.len() - 1 {
        if engine[y + 1][x].parse::<u32>().is_ok() { 
            adjecent.push(engine[y + 1][x].parse::<u32>().unwrap()); 
        } else {
            if x > 0 {
                if engine[y + 1][x - 1].parse::<u32>().is_ok() { 
                    adjecent.push(engine[y + 1][x - 1].parse::<u32>().unwrap()); 
                }
            }
            if x < row.len() - 1 {
                if engine[y + 1][x + 1].parse::<u32>().is_ok() { 
                    adjecent.push(engine[y + 1][x + 1].parse::<u32>().unwrap()); 
                }
            }
        }
    }

    // check left
    if x > 0 {
        if engine[y][x - 1].parse::<u32>().is_ok() { 
            adjecent.push(engine[y][x - 1].parse::<u32>().unwrap()); 
        }
    }
    
    // check right
    if x < row.len() - 1 {
        if engine[y][x + 1].parse::<u32>().is_ok() { 
            adjecent.push(engine[y][x + 1].parse::<u32>().unwrap()); 
        }
    }

    adjecent
}