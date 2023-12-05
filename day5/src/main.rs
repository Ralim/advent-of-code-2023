#![feature(btree_cursors)]
use std::{collections::BTreeMap, fs::read_to_string, ops::Bound};

#[derive(Debug, Clone)]
struct RangeMapper {
    range_a_start: u64,
    range_b_start: u64,
    length: u64,
}
impl RangeMapper {
    pub fn from_str(input: &str) -> Self {
        //Each line is 3 numbers
        // Range_a_start Range_b_start length
        let set = string_to_split_nums(input);
        Self {
            range_a_start: set[1],
            range_b_start: set[0],
            length: set[2],
        }
    }
    pub fn get_range_start(&self) -> u64 {
        return self.range_a_start;
    }
    pub fn convert_a_key_to_b_key_or_passthrough(&self, value: u64) -> u64 {
        if value > (self.range_a_start + self.length - 1) {
            value
        } else {
            self.range_b_start + (value - self.range_a_start)
        }
    }
}
#[derive(Debug, Clone)]
struct RangeSet {
    rangers: BTreeMap<u64, RangeMapper>,
}
impl RangeSet {
    fn insert(&mut self, value: RangeMapper) {
        self.rangers.insert(value.get_range_start(), value);
    }
    pub fn lookup(&self, value: u64) -> u64 {
        //Lookup
        let base_element = self.rangers.upper_bound(Bound::Included(&value));
        match base_element.key() {
            Some(matched_key) => {
                let ranger = &self.rangers[matched_key];
                ranger.convert_a_key_to_b_key_or_passthrough(value)
            }
            None => value, // Yield value through if no match
        }
    }

    pub fn insert_line(&mut self, line: &str) {
        //Parse a given line into a ranger and insert it
        self.insert(RangeMapper::from_str(line))
    }
}
impl Default for RangeSet {
    fn default() -> Self {
        Self {
            rangers: Default::default(),
        }
    }
}
fn string_to_split_nums(chunk_in: &str) -> Vec<u64> {
    let mut res = Vec::new();
    for s in chunk_in.split(' ') {
        let safe = s.trim();
        if safe.len() > 0 {
            res.push(s.parse().unwrap());
        }
    }
    res
}

fn read_file(filename: &str) -> u64 {
    let mut rangers: Vec<RangeSet> = Vec::new();
    let mut seeds = Vec::new();
    let mut current_range_set = RangeSet::default();

    for line in read_to_string(filename).unwrap().lines() {
        if seeds.len() == 0 {
            // If seeds is empty we are on first line, split it and yeet them into the vector
            let parts: Vec<&str> = line.split(':').collect();
            seeds = string_to_split_nums(parts[1]);
        } else {
            //We are in the following sections
            //If line is blank, break the current ranger and start a new one
            let cline = line.trim();
            if cline.len() == 0 {
                //Line break
                rangers.push(current_range_set);
                current_range_set = RangeSet::default();
            } else if cline.chars().next().unwrap().is_ascii_digit() {
                //Digit line, ingest into the map
                current_range_set.insert_line(cline);
            }
        }
    }
    rangers.push(current_range_set);

    // We now have all of our rangers; we now need to lookup each seed through the ranges
    //For part 2, these are run-length encoded

    let mut lowest_location = 0xFFFFFFFF;
    for seed_pair in seeds.chunks(2) {
        for seed in seed_pair[0]..seed_pair[0] + seed_pair[1] {
            let mut current_value = seed;
            for stage in &rangers {
                current_value = stage.lookup(current_value);
            }
            if current_value < lowest_location {
                lowest_location = current_value;
            }
        }
    }
    lowest_location
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
