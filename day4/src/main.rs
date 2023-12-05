use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone)]
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
fn collect_games(games: &[GameRecord], start_index: usize) -> Vec<GameRecord> {
    let mut res = Vec::new();
    let matched_numbers = games[start_index].get_matching_numbers().len();
    res.push(games[start_index].clone());
    for index_offset in 1..matched_numbers+1 {
        res.extend(collect_games(games, start_index + index_offset));
    }
    res
}
fn read_file_to_cards(filename: &str) -> usize {
    let mut games = Vec::new();
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let game = parse_card_line(line);
        games.push(game);
    }
    // Walk all games, find out how many numbers matched, add that many games to total
    for i in 0..games.len() {
        result.extend(collect_games(&games, i));
    }
    // println!("{:?}", result);
    result.len()
}

fn main() {
    let line_results = read_file_to_cards("input");

    println!("Total {}", line_results);
}
