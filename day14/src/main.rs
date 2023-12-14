use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::fs::read_to_string;

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
        let mut dirt_start = 0xFFFF;

        for i in 0..self.rock_set.len() {
            match self.rock_set[i] {
                GroundState::SquareRock => {
                    //Cant be moved, so have to flush queued dirt then insert
                    if dirt_start != 0xFFFF {
                        for x in dirt_start..i {
                            self.rock_set[x] = GroundState::Dirt
                        }
                        dirt_start = 0xFFFF;
                    }
                }
                GroundState::RoundRock => {
                    if dirt_start == 0xFFFF {
                        //Cant move, leave in place
                    } else {
                        self.rock_set[dirt_start] = GroundState::RoundRock;
                        dirt_start += 1;
                    }
                }
                GroundState::Dirt => {
                    if dirt_start == 0xFFFF {
                        dirt_start = i;
                    }
                }
            }
        }
        if dirt_start != 0xFFFF {
            for x in dirt_start..self.rock_set.len() {
                self.rock_set[x] = GroundState::Dirt
            }
        }
    }
    pub fn get_north_weight(&self) -> usize {
        let mut sum = 0;
        for (i, v) in self.rock_set.iter().enumerate() {
            if *v == GroundState::RoundRock {
                sum += self.rock_set.len() - i;
            }
        }
        sum
    }
}

#[derive(Clone, PartialEq)]
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
        for m in self.stripes.iter().rev() {
            m.print();
        }
        println!("<------");
    }
    pub fn slide_all_rocks_up(&mut self) {
        self.stripes
            .iter_mut()
            .for_each(|x| x.move_round_rocks_up());
    }
    pub fn get_north_weight(&self) -> usize {
        self.stripes.iter().map(|x| x.get_north_weight()).sum()
    }
    pub fn get_rotated_cw(&mut self) {
        let mut stripes = Vec::with_capacity(self.stripes[0].rock_set.len());

        for _ in 0..self.stripes[0].rock_set.len() {
            stripes.push(VerticalStripe::default());
        }

        for stripe in &self.stripes {
            for (i, v) in stripe.rock_set.iter().rev().enumerate() {
                stripes[i].rock_set.push(*v);
            }
        }
        self.stripes = stripes;
    }
}
fn rotate_one_iter(grid: &mut Grid) {
    grid.slide_all_rocks_up();
    grid.get_rotated_cw();
    grid.slide_all_rocks_up();
    grid.get_rotated_cw();
    grid.slide_all_rocks_up();
    grid.get_rotated_cw();
    grid.slide_all_rocks_up();
    grid.get_rotated_cw()
}
fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut grid: Grid = Grid::from_lines(&lines);
    grid.print();
    for i in 0..1_000_000_000 {
        rotate_one_iter(&mut grid);
        if i % 1_000_000 == 0 {
            println!("i {}", i);
        }
    }
    grid.get_north_weight()
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
