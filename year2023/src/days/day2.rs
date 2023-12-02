use std::collections::HashMap;
use crate::utils;

pub fn run(part: i8, file: String) -> String {
    println!("Running Part{part}:");
    match part {
        1 => part1(file),
        2 => part2(file),
        _ => "Part {part} not implemented".to_string(),
    }
}

fn part1(file: String) -> String {
    let lines = utils::read_lines(format!("./src/files/day2/{file}.txt").as_str());
    let games = parse_games(lines);
    let mut total = 0;

    for (i, game) in games.into_iter().enumerate() {
        if game["red"] <= 12 && game["green"] <= 13 && game["blue"] <= 14 {
            total += i + 1;
        }
    }

    total.to_string()
}

fn part2(file: String) -> String {
    let lines = utils::read_lines(format!("./src/files/day2/{file}.txt").as_str());
    let games = parse_games(lines);
    let mut total: u32 = 0;

    for game in games {
        total += game["red"] * game["green"] * game["blue"];
    }

    total.to_string()
}

fn parse_games(lines: Vec<String>) -> Vec<HashMap<String, u32>> {
    let mut games: Vec<HashMap<String, u32>> =  Vec::new();

    for (i, line) in lines.into_iter().enumerate() {
        let game_row: Vec<&str> = line.split(": ").collect();
        let game: Vec<&str> = game_row[1].split("; ").collect();
        games.insert(i, HashMap::new());

        for hands in game {
            let hand: Vec<&str> = hands.split(", ").collect();

            for colors in hand {
                let group: Vec<&str> = colors.split(" ").collect();
                let number = group[0].parse::<u32>().unwrap();
                let color = group[1];
                
                games[i].entry(color.to_string())
                    .and_modify(|x| { if *x < number {*x = number;}})
                    .or_insert_with(|| number);
            }
        }   
    }

    games
}