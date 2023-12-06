use crate::{utils, Day};

pub struct Day4 {}

struct Card {
    copies: u32,
    winning_numbers: Vec<u8>,
    current_numbers: Vec<u8>,
}

impl Day for Day4 {
    fn part1(&self, file: String) -> String {
        let cards = get_cards(file);
        let powpow: u32 = 2;
        let mut points = 0;
        let mut winners;

        for card in cards {
            winners = Vec::new();

            for winner in card.winning_numbers {
                if card.current_numbers.contains(&winner) {
                    winners.push(winner);
                }
            }

            if winners.len() > 0 {
                points += powpow.pow(winners.len() as u32 - 1);
            }

        }

        points.to_string()
    }

    fn part2(&self, file: String) -> String {
        let mut cards = get_cards(file);
        let mut total_cards = cards.len();
        let mut winners;
        let mut card;

        for card_index in 0..cards.len() {
            winners = Vec::new();
            card = &cards[card_index];

            for winner in card.winning_numbers.as_slice() {
                if card.current_numbers.contains(&winner) {
                    winners.push(winner);
                }
            }

            if winners.len() > 0 {
                for x in 1..winners.len() + 1 {
                    cards[card_index + x].copies += cards[card_index].copies;
                    total_cards += cards[card_index].copies as usize;
                }
            }
        }

        total_cards.to_string()
    }
}

fn get_cards(file: String) -> Vec<Card> {
    let lines = utils::read_lines(format!("./src/files/day4/{file}.txt").as_str());
    let mut cards: Vec<Card> = Vec::new();


    for line in lines {
        let card: Vec<&str> = line.split(":").collect(); 
        let numbers: Vec<&str> = card[1].split(" |").collect();

        let winners: Vec<u8> = numbers[0]
            .split_ascii_whitespace()
            .map(|i| i.trim().parse::<u8>().unwrap())
            .collect();

        let current = numbers[1]
            .split_ascii_whitespace()
            .map(|i| i.trim().parse::<u8>().unwrap())
            .collect();

        cards.push(Card {copies: 1, winning_numbers: winners, current_numbers: current});
    }

    cards
}