use rayon::prelude::*;
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

enum GroundState {
    SquareRock,
    RoundRock,
    Dirt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerticalStripe {
    pub rock_set: Vec<GroundState>,
}
impl Default for VerticalStripe {
    fn default() -> Self {
        Self {
            rock_set: Vec::with_capacity(100),
        }
    }
}

impl VerticalStripe {
    pub fn ingest(&mut self, data: char) {
        self.rock_set.push(match data {
            '.' => GroundState::Dirt,
            '#' => GroundState::SquareRock,
            'O' => GroundState::RoundRock,
            _ => panic!("Bad ground type"),
        });
    }
    pub fn print(&self) {
        for v in &self.rock_set {
            match v {
                GroundState::SquareRock => print!("#"),
                GroundState::RoundRock => print!("O"),
                GroundState::Dirt => print!("."),
            }
        }
        println!();
    }
    pub fn move_round_rocks_up(&mut self) {
        let queue = VecDeque::from_iter(self.rock_set.iter().cloned());
        self.rock_set.clear();

        let mut push_back = VecDeque::with_capacity(10);
        for value in queue {
            match value {
                GroundState::SquareRock => {
                    //Cant be moved, so have to flush queue then insert
                    self.rock_set.extend(push_back.iter());
                    push_back.clear();
                    self.rock_set.push(value);
                }
                GroundState::RoundRock => {
                    self.rock_set.push(value);
                }
                GroundState::Dirt => {
                    push_back.push_back(value);
                }
            }
        }
        self.rock_set.extend(push_back);
    }
    pub fn get_north_weight(&self) -> usize {
        let mut sum = 0;
        for (i, v) in self.rock_set.iter().enumerate() {
            if *v == GroundState::RoundRock {
                sum += (self.rock_set.len() - i);
            }
        }
        sum
    }
}

#[derive(Clone)]
struct Grid {
    stripes: Vec<VerticalStripe>,
}

impl Grid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut stripes = Vec::new();

        for _ in 0..lines[0].len() {
            stripes.push(VerticalStripe::default());
        }

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                stripes[i].ingest(c);
            }
        }

        Self { stripes }
    }

    pub fn print(&self) {
        println!("<------");
        for m in &self.stripes {
            m.print();
        }
        println!("<------");
    }
    pub fn slide_all_rocks_up(&mut self) {
        for x in self.stripes.iter_mut() {
            x.move_round_rocks_up();
        }
    }
    pub fn get_north_weight(&self) -> usize {
        self.stripes.iter().map(|x| x.get_north_weight()).sum()
    }
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut grid: Grid = Grid::from_lines(&lines);
    grid.print();

    grid.slide_all_rocks_up();
    grid.print();
    grid.get_north_weight()
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
