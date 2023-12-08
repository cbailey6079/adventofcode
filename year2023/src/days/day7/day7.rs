use std::cmp::Ordering;
use itertools::Itertools;


use crate::{utils, Day};

pub struct Day7 {}

#[derive(PartialEq, Eq, Debug)]
struct Hand1 {
    hand: String
}

impl Hand1 {
    fn c_cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left:  Vec<&str> = other.hand.split("").collect();
        let right: Vec<&str> = self.hand.split("").collect();

        for (i, l) in left.iter().enumerate() {
            if *l == right[i] { continue; }

            if *l == "J".to_string() && (right[i] == "K".to_string() || right[i] == "Q".to_string()) {
                return Ordering::Less;
            } else if  right[i] == "J".to_string() && (*l == "K".to_string() || *l == "Q".to_string()) {
                return Ordering::Greater;
            } else if l.parse::<u8>().is_err() && right[i].parse::<u8>().is_err() {
                return right[i].cmp(l);
            } else {      
                return l.cmp(&right[i]);
            }
        }

        Ordering::Equal
    }

    fn j_cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left:  Vec<&str> = other.hand.split("").collect();
        let right: Vec<&str> = self.hand.split("").collect();

        for (i, l) in left.iter().enumerate() {
            if *l == right[i] { continue; }

            if *l == "J".to_string() { 
                return Ordering::Less;
            } else if right[i] == "J".to_string() { 
                return Ordering::Greater;
            } else if l.parse::<u8>().is_err() && right[i].parse::<u8>().is_err() {
                return right[i].cmp(l);
            } else {      
                return l.cmp(&right[i]);
            }
        }

        Ordering::Equal
    }
}

#[derive(Debug)]
struct Hand<'a> {
    compare_hand: Hand1,
    hand: Vec<&'a str>,
    rank: u8,
    bid: u16,
}

impl Day for Day7 {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day7/files/{file}.txt").as_str());

        let mut hands: Vec<Hand> = lines.iter()
            .map(|line| { 
                let things: Vec<&str> = line.split(" ").collect(); 
                Hand {compare_hand: Hand1 { hand: things[0].to_string() }, hand: things[0].split("").filter(|a| a != &"").collect_vec(), bid: things[1].parse::<u16>().unwrap(), rank: 1 }
            }).collect();

        let mut winnings = 0;

        for curr_hand in &mut hands {
            let mut temp_hand = curr_hand.hand.clone();
            temp_hand.sort();

            let mut hand_count = temp_hand
                .iter()
                .group_by(|c| *c)
                .into_iter()
                .map(|g| (g.0, g.1.count()))
                .collect_vec();

            hand_count.sort_by(|a , b| b.1.cmp(&a.1));

            match hand_count[0].1 {
                5 => curr_hand.rank = 7,
                4 => curr_hand.rank = 6,
                3 => match hand_count[1].1 {
                    2 => curr_hand.rank = 5,
                    _ => curr_hand.rank = 4,
                },
                2 => match hand_count[1].1 {
                    2 => curr_hand.rank = 3,
                    _ => curr_hand.rank = 2,
                },
                _ => continue,
            }
        } 

        let mut starting_rank = hands.len();

        for i in (1..8).rev() {
            let mut all_ranks: Vec<&Hand> = hands.iter().filter(|hand| hand.rank == i).collect_vec();

            all_ranks.sort_by(|&a, &b| a.compare_hand.c_cmp(&b.compare_hand));

            for &item in &all_ranks {
                winnings += item.bid as usize * starting_rank;
                starting_rank -= 1;
            }
        }

        winnings.to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/day7/files/{file}.txt").as_str());

        let mut hands: Vec<Hand> = lines.iter()
            .map(|line| { 
                let things: Vec<&str> = line.split(" ").collect(); 
                Hand {compare_hand: Hand1 { hand: things[0].to_string() }, hand: things[0].split("").filter(|a| a != &"").collect_vec(), bid: things[1].parse::<u16>().unwrap(), rank: 1 }
            }).collect();

        let mut winnings = 0;

        for curr_hand in &mut hands {
            let mut temp_hand = curr_hand.hand.clone();
            temp_hand.sort();

            let mut hand_count = temp_hand
                .iter()
                .group_by(|c| *c)
                .into_iter()
                .map(|g| (g.0, g.1.count()))
                .collect_vec();

            hand_count.sort_by(|a , b| b.1.cmp(&a.1));

            let jack_count = temp_hand.iter().filter(|c| **c == "J".to_string()).collect::<Vec<&&str>>().len();
            let mut match_me = hand_count[0].1;

            if hand_count[0].0 != &"J" {
                match_me += jack_count;
            } else if hand_count.len() > 1 && hand_count[0].0 == &"J" {
                match_me += hand_count[1].1;
            }
            
            match match_me {
                5 => curr_hand.rank = 7,
                4 => curr_hand.rank = 6,
                3 => match hand_count[1].1 {
                    2 => curr_hand.rank = 5,
                    _ => curr_hand.rank = 4,
                },
                2 => match hand_count[1].1 {
                    2 => curr_hand.rank = 3,
                    _ => curr_hand.rank = 2,
                },
                _ => continue,
            }
        } 

        let mut starting_rank = hands.len();

        for i in (1..8).rev() {
            let mut all_ranks: Vec<&Hand> = hands.iter().filter(|hand| hand.rank == i).collect_vec();

            all_ranks.sort_by(|&a, &b| a.compare_hand.j_cmp(&b.compare_hand));

            for &item in &all_ranks {
                winnings += item.bid as usize * starting_rank;
                starting_rank -= 1;
            }
        }

        winnings.to_string()
    }
}


#[cfg(test)]
#[test]
fn day7_part1_example() {
    let day = Day7 {};
    let expected = "6440";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day7_part1() {
    let day = Day7 {};
    let expected = "246424613";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day7_part2_example() {
    let day = Day7 {};
    let expected = "5905";

    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn day7_part2() {
    let day = Day7 {};
    let expected = "248256639";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}