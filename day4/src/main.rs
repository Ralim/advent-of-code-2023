use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct GameRecord {
    card_number: u32,
    winners: HashSet<u32>,
    drawn: Vec<u32>,
}
impl GameRecord {
    pub fn get_matching_numbers(&self) -> Vec<u32> {
        let mut res = Vec::new();

        for game in &self.drawn {
            if self.winners.contains(game) {
                res.push(*game);
            }
        }
        res
    }
    pub fn get_game_score(&self) -> u32 {
        let matching_numbers = self.get_matching_numbers();
        if matching_numbers.len() == 0 {
            return 0;
        }
        let mut score = 1;
        for _ in 1..matching_numbers.len() {
            score *= 2;
        }
        return score;
    }
}

fn extract_card_number(input: &str) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Card +(?P<card_id>[\d]+)").unwrap();
    }
    RE.captures(input)
        .and_then(|cap| {
            cap.name("card_id")
                .map(|card_id| card_id.as_str().parse::<u32>().unwrap())
        })
        .unwrap()
}
fn string_to_split_nums(chunk_in: &str) -> Vec<u32> {
    let mut res = Vec::new();
    for s in chunk_in.split(' ') {
        let safe = s.trim();
        if safe.len() > 0 {
            res.push(s.parse().unwrap());
        }
    }
    res
}
fn parse_card_line(chunk: &str) -> GameRecord {
    let label: Vec<&str> = chunk.split(':').collect();
    let card_number = extract_card_number(label[0]);
    let numbers_field: Vec<&str> = label[1].split('|').collect();
    let winners = string_to_split_nums(numbers_field[0]);

    let drawn = string_to_split_nums(numbers_field[1]);
    GameRecord {
        card_number,
        winners: HashSet::from_iter(winners),
        drawn,
    }
}

fn read_file_to_cards(filename: &str) -> Vec<u32> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let game = parse_card_line(line);
        result.push(game.get_game_score());
        println!("Counted");
    }
    result
}

fn main() {
    let line_results = read_file_to_cards("input");
    let mut sum = 0;
    for v in line_results {
        sum += v;
    }
    println!("Total {}", sum);
}
