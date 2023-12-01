use phf::phf_map;
use std::{collections::HashMap, fs::read_to_string};

static LOOKUP_TABLE: phf::Map<&'static str, u32> = phf_map! {
    "0"=> 0,
    "1"=> 1,
    "2"=> 2,
    "3"=> 3,
    "4"=> 4,
    "5"=> 5,
    "6"=> 6,
    "7"=> 7,
    "8"=> 8,
    "9"=> 9,
    "one"=> 1,
    "two"=> 2,
    "three"=> 3,
    "four"=> 4,
    "five"=> 5,
    "six"=> 6,
    "seven"=> 7,
    "eight"=> 8,
    "nine"=> 9,
};

fn match_string_start(line: &str) -> u32 {
    let mut best_guess_index = line.len() + 1;
    let mut best_guess_value = 0;
    for (k, v) in &LOOKUP_TABLE {
        match line.find(k) {
            None => {}
            Some(index) => {
                //

                if index < best_guess_index {
                    best_guess_index = index;
                    best_guess_value = *v;
                }
            }
        }
    }
    if best_guess_index < line.len() {
        return best_guess_value;
    }
    panic!("No Matcher?")
}
fn match_string_tail(line: &str) -> u32 {
    let mut best_guess_index: isize = -1;
    let mut best_guess_value = 0;
    for (k, v) in &LOOKUP_TABLE {
        match line.rfind(k) {
            None => {}
            Some(i) => {
                //
                let index = i as isize;

                if index > best_guess_index {
                    best_guess_index = index;
                    best_guess_value = *v;
                }
            }
        }
    }
    if best_guess_index > -1 {
        return best_guess_value;
    }
    panic!("No Matcher?")
}
fn read_file_to_tailing_digits(filename: &str) -> Vec<u32> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let tens = match_string_start(&line.to_string());
        let units = match_string_tail(&line.to_string());
        let ans = tens * 10 + units;
        // println!("{} -> {}", line, ans);
        result.push(ans)
    }
    result
}

fn main() {
    let line_results = read_file_to_tailing_digits("input");
    let mut sum = 0;
    for v in line_results {
        sum += v;
    }
    println!("Total {}", sum);
}
