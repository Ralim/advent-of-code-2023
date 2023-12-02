use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct WithdrawnSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct GameRecord {
    game_id: u32,
    withdrawals: Vec<WithdrawnSet>,
}
impl GameRecord {
    pub fn get_max_seen(&self) -> WithdrawnSet {
        let mut res = WithdrawnSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for game in &self.withdrawals {
            res.red = res.red.max(game.red);
            res.green = res.green.max(game.green);
            res.blue = res.blue.max(game.blue);
        }
        res
    }
}

fn extract_game_number(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Game (?P<game_id>[\d]+)").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("game_id")
            .map(|game_id| game_id.as_str().parse::<u32>().unwrap())
    })
}
fn game_color_chunk_split(chunk_in: &str) -> (&str, u32) {
    let chunk = chunk_in.trim();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<count>[\d]+) (?P<colour>[\D]+)").unwrap();
    }
    let colour = RE
        .captures(chunk)
        .and_then(|cap| cap.name("colour").map(|colour| colour.as_str()))
        .unwrap();
    let count = RE
        .captures(chunk)
        .and_then(|cap| {
            cap.name("count")
                .map(|count| count.as_str().parse::<u32>().unwrap())
        })
        .unwrap();
    return (colour, count);
}
fn parse_game_set(chunk: &str) -> WithdrawnSet {
    //Given: 1 green, 3 red, 6 blue
    let mut result = WithdrawnSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    let chunks = chunk.split(',');
    for chunk in chunks {
        let (colour, count) = game_color_chunk_split(chunk);
        match colour {
            "red" => result.red += count,
            "green" => result.green += count,
            "blue" => result.blue += count,
            _ => panic!("Unhandled colour {}", colour),
        };
    }
    result
}
fn read_game_line_to_parts(line: &str) -> GameRecord {
    let mut result = GameRecord {
        game_id: 0,
        withdrawals: Vec::new(),
    };
    //Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    let split_title: Vec<&str> = line.split(':').collect();
    if split_title.len() != 2 {
        panic!("Bad split game {:?}", split_title);
    }
    //Grab from the split title the game number
    result.game_id = extract_game_number(split_title[0]).unwrap();
    for chunk in split_title[1].split(';') {
        result.withdrawals.push(parse_game_set(chunk));
    }
    result
}

fn read_file_lines_to_possible_games(filename: &str) -> Vec<u32> {
    let mut result = Vec::new();
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    for line in read_to_string(filename).unwrap().lines() {
        let game = read_game_line_to_parts(line);
        let sum = game.get_max_seen();

        println!("Game -> {:?} = {:?}", game.game_id, sum);
        if sum.red <= max_red && sum.green <= max_green && sum.blue <= max_blue {
            result.push(game.game_id);
            println!("Counted");
        }
    }
    result
}

fn main() {
    let line_results = read_file_lines_to_possible_games("input");
    let mut sum = 0;
    for v in line_results {
        sum += v;
    }
    println!("Total {}", sum);
}
