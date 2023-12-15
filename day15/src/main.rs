use std::fs::read_to_string;

use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// #[memoize(Capacity: 1_000_000)]

fn hash_inner(mut value: usize, input: char) -> usize {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    let val = input as usize;
    if input < ' ' {
        return value;
    }
    value += val;

    value *= 17;
    value = value % 256;

    value
}
fn hash_str(input: &str) -> usize {
    let mut sum = 0;

    for c in input.chars() {
        sum = hash_inner(sum, c);
    }
    sum
}
fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();

    let sum = file_contents
        .split(',')
        .map(|i| {
            let res = hash_str(i);
            println!("{} -> {}", i, res);
            res
        })
        .sum();
    sum
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
