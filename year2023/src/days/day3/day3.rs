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
    let lines = utils::read_lines(format!("./src/days/day3/files/{file}.txt").as_str());

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

fn check_adjacent(engine: &Vec<Vec<String>>, current_point: (usize,usize)) -> Vec<u32> {
    let (y, x) = current_point;
    let mut adjacent: Vec<u32> = Vec::new();

    let locations: Vec<(usize,usize)> = [
        (y - 1, x), 
        (y - 1, x - 1), 
        (y - 1, x + 1),
        (y + 1, x),
        (y + 1, x - 1),
        (y + 1, x + 1),
        (y, x - 1),
        (y, x + 1)
        ].to_vec();

    for location in locations {
        match engine.get(location.0) {
            Some(row) => {
                match row.get(location.1) {
                    Some(item) => {
                        if item.parse::<u32>().is_ok() { 
                            adjacent.push(item.parse::<u32>().unwrap()); 
                        }
                    },
                    None => continue,
                }
            },
            None => continue,
        };
    }

    adjacent.dedup();
    
    adjacent
}


#[cfg(test)]
#[test]
fn day3_part1_example() {
    let day = Day3 {};
    let expected = "4361";
    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day3_part1() {
    let day = Day3 {};
    let expected = "536202";
    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day3_part2_example() {
    let day = Day3 {};
    let expected = "467835";
    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day3_part2() {
    let day = Day3 {};
    let expected = "78272573";
    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}