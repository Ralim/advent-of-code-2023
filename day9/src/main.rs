use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct NumberSeries {
    number_set: Vec<i64>,
}

impl NumberSeries {
    pub fn from_str(value: &str) -> Self {
        let number_set: Vec<i64> = value.split(' ').map(|s| s.parse().unwrap()).collect();

        Self { number_set }
    }
    pub fn from_slice(values: &[i64]) -> Self {
        Self {
            number_set: Vec::from_iter(values.iter().map(|f| *f)),
        }
    }
    pub fn get_derivative(&self) -> NumberSeries {
        let mut res = Vec::new();

        for i in 1..self.number_set.len() {
            res.push(self.number_set[i] - self.number_set[i - 1]);
        }
        Self::from_slice(&res)
    }
    pub fn is_zero(&self) -> bool {
        for v in &self.number_set {
            if *v != 0 {
                return false;
            }
        }
        true
    }
    //Insert new value, given the value of the below number

    pub fn get_last(&self) -> i64 {
        *self.number_set.last().unwrap()
    }
    pub fn get_first(&self) -> i64 {
        *self.number_set.first().unwrap()
    }
}
fn expand_set_return_last(set: &NumberSeries) -> i64 {
    // println!("Set {:?}", set);
    if set.is_zero() {
        return 0;
    }
    let mut sub_set = set.get_derivative();
    let last_number = expand_set_return_last(&mut sub_set);
    return set.get_last() + last_number;
}

fn prepend_set_return_last(set: &NumberSeries) -> i64 {
    // println!("Set {:?}", set);
    if set.is_zero() {
        return 0;
    }
    let mut sub_set = set.get_derivative();
    let last_number = prepend_set_return_last(&mut sub_set);
    return set.get_first() - last_number;
}

fn read_file(filename: &str) -> i64 {
    let file_contents = read_to_string(filename).unwrap();
    let number_sets: Vec<NumberSeries> = file_contents
        .lines()
        .map(|l| NumberSeries::from_str(l))
        .collect();

    let mut sum_added: i64 = 0;
    for set in &number_sets {
        // For each set, depth recurse to find 0's, then expand and go back up the stack
        // let last_number = expand_set_return_last(set);
        let last_number = prepend_set_return_last(set);
        println!("Last {}", last_number);
        sum_added += last_number;
    }
    sum_added
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
