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
}

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
    pub fn is_mirror_line(&self, row: usize) -> bool {
        let mut top_half = &self.mirror_sets[0..row];
        let mut bottom_half = &self.mirror_sets[row + 2..self.mirror_sets.len()];
        // Trim these to the same length
        if top_half.len() > bottom_half.len() {
            top_half = &top_half[(top_half.len() - bottom_half.len())..top_half.len()];
        } else if top_half.len() < bottom_half.len() {
            bottom_half = &bottom_half[0..top_half.len()];
        }
        //Flip one and compare
        for (a, b) in top_half.iter().rev().zip(bottom_half.iter()) {
            println!("Compare {:?} {:?}", a, b);
            if !a.matches(b) {
                return false;
            }
        }
        true
    }
    pub fn get_number_of_rows_above_mirror_line(&self) -> usize {
        let mut sum_of_rows_above = 0;
        for row in 0..self.mirror_sets.len() - 1 {
            if self.mirror_sets[row].matches(&self.mirror_sets[row + 1]) {
                //We found a potential mirror point
                if self.is_mirror_line(row) {
                    sum_of_rows_above += row + 1;

                    println!("Mirror line at {}", row);
                }
            }
        }
        sum_of_rows_above
    }
    pub fn get_rotated_ccw(&self) -> Self {
        let mut mirror_sets = Vec::new();
        for _ in &self.mirror_sets[0].spring_state {
            mirror_sets.push(MirrorLine {
                spring_state: Vec::new(),
            });
        }

        for (row, mirror) in self.mirror_sets.iter().rev().enumerate() {
            for (col, val) in mirror.spring_state.iter().enumerate() {
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
    let grids: Vec<MirrorGrid> = MirrorGrid::from_lines(&lines);
    let grid_mirrors: Vec<MirrorGrid> = grids.iter().map(|x| x.get_rotated_ccw()).collect();

    let horizontal_mirrors: usize = grids
        .iter()
        .map(|g| {
            g.print();
            let rows = g.get_number_of_rows_above_mirror_line();
            println!("Rows above mirror line {}", rows);
            rows
        })
        .sum();

    let vertical_mirrors: usize = grid_mirrors
        .iter()
        .map(|g| {
            g.print();
            let rows = g.get_number_of_rows_above_mirror_line();
            println!("Rows above mirror line {}", rows);
            rows
        })
        .sum();

    println!(
        "Vertical mirrors {}, horizontal mirrors {}",
        vertical_mirrors, horizontal_mirrors
    );

    vertical_mirrors + (100 * horizontal_mirrors)
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
