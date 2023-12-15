use jemallocator::Jemalloc;
use rayon::prelude::*;
use std::{fs::read_to_string, time::Instant};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Debug, Clone, PartialEq, Eq)]

enum SpringState {
    Unknown,
    Working,
    Broken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpringLine {
    pub spring_state: Vec<SpringState>,
    pub outage_set: Vec<usize>,
}
fn count_outages(set: &Vec<SpringState>, counted_outages: &mut Vec<usize>, max_len: usize) {
    counted_outages.clear();
    let mut counter = 0;
    for state in set {
        match *state {
            SpringState::Unknown => {
                counter = 0;
                break;
            }
            SpringState::Working => {
                if counter > 0 {
                    if (counted_outages.len() + 1) > max_len {
                        return;
                    }
                    counted_outages.push(counter);
                    counter = 0;
                }
            }
            SpringState::Broken => {
                counter += 1;
            }
        }
    }
    if counter > 0 {
        counted_outages.push(counter);
    }
}

impl SpringLine {
    pub fn expand_part_2(&mut self) {
        // To unfold the records, on each row,
        // replace the list of spring conditions with five copies of itself (separated by ?)
        // and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).

        let base = self.clone();
        for _ in 0..4 {
            self.spring_state.push(SpringState::Unknown);
            self.spring_state.extend_from_slice(&base.spring_state);
            self.outage_set.extend_from_slice(&base.outage_set);
        }
    }

    pub fn is_valid_until_unknown(&self, counted_outages: &mut Vec<usize>) -> (bool, bool) {
        //So we count the sequence of broken springs, this should match the outage set

        count_outages(&self.spring_state, counted_outages, self.outage_set.len());
        // over length so immediately wrong
        if counted_outages.len() > self.outage_set.len() {
            return (false, false);
        }

        for (a, b) in (counted_outages.iter().zip(self.outage_set.iter())).rev() {
            if *a != *b {
                return (false, false);
            }
        }

        (true, counted_outages.len() == self.outage_set.len())
    }

    pub fn count_possible_solutions(&self, scratch: &mut Vec<usize>) -> usize {
        // Find the first unknown, and fork it out
        let (valid_fast, valid_full_check) = self.is_valid_until_unknown(scratch);
        if !valid_fast {
            return 0;
        }
        let index_of_unknown = self
            .spring_state
            .iter()
            .position(|r| *r == SpringState::Unknown);

        match index_of_unknown {
            Some(index) => {
                let mut working = self.clone();
                let mut broken = self.clone();
                working.spring_state[index] = SpringState::Working;
                broken.spring_state[index] = SpringState::Broken;

                working.count_possible_solutions(scratch) + broken.count_possible_solutions(scratch)
            }
            None => {
                if valid_full_check {
                    return 1;
                }
                return 0;
            }
        }
    }
}
impl Default for SpringLine {
    fn default() -> Self {
        Self {
            spring_state: Vec::new(),
            outage_set: Vec::new(),
        }
    }
}
struct SpringGrid {
    spring_sets: Vec<SpringLine>,
}

impl SpringGrid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut spring_sets: Vec<SpringLine> = Vec::new();

        for line in lines.iter() {
            let splits: Vec<&str> = line.split(' ').collect();
            let mut line = SpringLine::default();

            for c in splits[0].chars() {
                //
                if c == '.' {
                    line.spring_state.push(SpringState::Working);
                } else if c == '#' {
                    line.spring_state.push(SpringState::Broken);
                } else {
                    line.spring_state.push(SpringState::Unknown);
                }
            }
            line.outage_set = splits[1].split(',').map(|s| s.parse().unwrap()).collect();
            line.expand_part_2();
            spring_sets.push(line);
        }

        Self { spring_sets }
    }
    pub fn get_total_solutions(&self) -> usize {
        let par_iter = self.spring_sets.par_iter().map(|x| {
            let mut counted_outages: Vec<usize> = Vec::with_capacity(100);
            let start = Instant::now();
            let res = x.count_possible_solutions(&mut counted_outages);
            let time = Instant::now().duration_since(start).as_millis() as f64;
            println!("Line -> {} in {} seconds", res, time / 1000.0);
            res
        });

        par_iter.sum()
    }
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let grid = SpringGrid::from_lines(&lines);
    grid.get_total_solutions()
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
