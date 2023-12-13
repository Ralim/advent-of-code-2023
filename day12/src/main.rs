use rayon::prelude::*;
use std::fs::read_to_string;

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

impl SpringLine {
    pub fn is_complete(&self) -> bool {
        self.spring_state
            .iter()
            .filter(|p| **p == SpringState::Unknown)
            .count()
            == 0
    }
    pub fn is_valid(&self) -> bool {
        //So we count the sequence of broken springs, this should match the outage set
        let mut counted_outages: Vec<usize> = Vec::new();
        let mut counter = 0;
        for state in &self.spring_state {
            if *state == SpringState::Working {
                if counter > 0 {
                    counted_outages.push(counter);
                    counter = 0;
                }
            } else if *state == SpringState::Broken {
                counter += 1;
            }
        }
        if counter > 0 {
            counted_outages.push(counter);
        }
        // println!(
        //     "Row {:?} -> {:?} | {:?}",
        //     self.spring_state, self.outage_set, counted_outages
        // );

        counted_outages == self.outage_set
    }

    pub fn count_possible_solutions(&self) -> usize {
        // Find the first unknown, and fork it out
        if self.is_complete() {
            if !self.is_valid() {
                return 0;
            }
            return 1;
        }
        //Fork/split and sum
        let index = self
            .spring_state
            .iter()
            .position(|r| *r == SpringState::Unknown)
            .unwrap();
        let mut working = self.clone();
        let mut broken = self.clone();
        working.spring_state[index] = SpringState::Working;
        broken.spring_state[index] = SpringState::Broken;

        working.count_possible_solutions() + broken.count_possible_solutions()
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

            spring_sets.push(line);
        }

        Self { spring_sets }
    }
    pub fn get_total_solutions(&self) -> usize {
        let par_iter = self
            .spring_sets
            .par_iter()
            .map(|x| x.count_possible_solutions());

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
