use rayon::prelude::*;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

enum GroundState {
    Rock,
    Dirt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MirrorLine {
    pub spring_state: Vec<GroundState>,
}

impl MirrorLine {
    pub fn from_str(data: &str) -> Self {
        let spring_state: Vec<GroundState> = data
            .chars()
            .map(|c| {
                if c == '.' {
                    GroundState::Dirt
                } else {
                    GroundState::Rock
                }
            })
            .collect();
        Self { spring_state }
    }

    pub fn matches(&self, alt: &Self) -> bool {
        for (a, b) in self.spring_state.iter().zip(alt.spring_state.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
    pub fn delta(&self, alt: &Self) -> usize {
        let mut diff = 0;
        for (a, b) in self.spring_state.iter().zip(alt.spring_state.iter()) {
            if a != b {
                diff += 1;
            }
        }
        diff
    }
    pub fn flip_delta(&mut self, alt: &Self) {
        let mut flip_pos = 0;
        for (pos, (a, b)) in self
            .spring_state
            .iter()
            .zip(alt.spring_state.iter())
            .enumerate()
        {
            if a != b {
                flip_pos = pos
            }
        }
        self.spring_state[flip_pos] = if self.spring_state[flip_pos] == GroundState::Dirt {
            GroundState::Rock
        } else {
            GroundState::Dirt
        }
    }
}

#[derive(Clone)]
struct MirrorGrid {
    mirror_sets: Vec<MirrorLine>,
}

impl MirrorGrid {
    pub fn from_lines(lines: &[&str]) -> Vec<Self> {
        //
        let mut grids = Vec::new();
        let mut mirror_sets: Vec<MirrorLine> = Vec::new();

        for line in lines.iter() {
            if line.len() > 1 {
                mirror_sets.push(MirrorLine::from_str(line));
            } else if mirror_sets.len() > 0 {
                grids.push(MirrorGrid {
                    mirror_sets: mirror_sets.clone(),
                });
                mirror_sets.clear();
            }
        }
        if mirror_sets.len() > 0 {
            grids.push(MirrorGrid {
                mirror_sets: mirror_sets.clone(),
            });
            mirror_sets.clear();
        }
        grids
    }
    pub fn is_mirror_line(&mut self, row: usize) -> bool {
        let mut bottom_half: Vec<MirrorLine> = (&self.mirror_sets[row + 1..self.mirror_sets.len()])
            .iter()
            .cloned()
            .collect();
        //
        let mut top_half = &mut self.mirror_sets[0..(row + 1)];
        println!("{} Top half -> {:?}", row, top_half);
        println!("{} Bottom half -> {:?}", row, bottom_half);
        // Trim these to the same length
        if top_half.len() > bottom_half.len() {
            let len = top_half.len();
            top_half = &mut top_half[(len - bottom_half.len())..len];
        } else if top_half.len() < bottom_half.len() {
            bottom_half.truncate(top_half.len())
        }
        //Flip one and compare
        let mut sum_of_deltas = 0;
        for (a, b) in top_half.iter().rev().zip(bottom_half.iter()) {
            println!("Compare {:?} {:?}->{}", a, b, a.delta(b));
            sum_of_deltas += a.delta(b);
        }
        sum_of_deltas == 1
    }
    pub fn get_number_of_rows_above_mirror_line(&mut self) -> usize {
        for row in 0..self.mirror_sets.len() - 1 {
            if self.mirror_sets[row].matches(&self.mirror_sets[row + 1])
                || self.mirror_sets[row].delta(&self.mirror_sets[row + 1]) == 1
            {
                //We found a potential mirror point
                if self.is_mirror_line(row) {
                    println!("Mirror line at {}", row);

                    return row + 1;
                }
            }
        }
        0
    }

    pub fn get_rotated_ccw(&self) -> Self {
        let mut mirror_sets = Vec::new();
        for _ in &self.mirror_sets[0].spring_state {
            mirror_sets.push(MirrorLine {
                spring_state: Vec::new(),
            });
        }

        for mirror in self.mirror_sets.iter().rev() {
            for (col, val) in mirror.spring_state.iter().enumerate() {
                mirror_sets[col].spring_state.push(*val);
            }
        }

        Self { mirror_sets }
    }

    pub fn get_rotated_cw(&self) -> Self {
        let mut mirror_sets = Vec::new();
        for _ in &self.mirror_sets[0].spring_state {
            mirror_sets.push(MirrorLine {
                spring_state: Vec::new(),
            });
        }

        for mirror in self.mirror_sets.iter() {
            for (col, val) in mirror.spring_state.iter().rev().enumerate() {
                mirror_sets[col].spring_state.push(*val);
            }
        }

        Self { mirror_sets }
    }

    pub fn print(&self) {
        for m in &self.mirror_sets {
            let line: String = m
                .spring_state
                .iter()
                .map(|v| if *v == GroundState::Rock { '#' } else { '.' })
                .collect();
            println!("{}", line);
        }
        println!();
    }
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut grids: Vec<MirrorGrid> = MirrorGrid::from_lines(&lines);

    let mut horizontal_mirrors: usize = grids
        .iter_mut()
        .map(|g| {
            let rows = g.get_number_of_rows_above_mirror_line();
            println!("Rows above mirror line {}", rows);
            rows
        })
        .sum();

    let vertical_mirrors: usize = grids
        .iter_mut()
        .map(|g| {
            let mut rot = g.get_rotated_ccw();
            let rows = rot.get_number_of_rows_above_mirror_line();
            // println!("Rows above mirror line {}", rows);
            rows
        })
        .sum();

    println!(
        "vertical_mirrors {} horizontal_mirrors {}",
        vertical_mirrors, horizontal_mirrors
    );

    vertical_mirrors + (100 * horizontal_mirrors)
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
