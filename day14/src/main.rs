use array2d::Array2D;
use jemallocator::Jemalloc;
use memoize::memoize;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::{
    fmt::{self},
    fs::read_to_string,
    time::Instant,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]

enum GroundState {
    SquareRock,
    RoundRock,
    Dirt,
}
impl fmt::Display for GroundState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GroundState::SquareRock => write!(f, "#"),
            GroundState::RoundRock => write!(f, "O"),
            GroundState::Dirt => write!(f, "."),
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Grid {
    grid: Array2D<GroundState>,
}

impl Grid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut stripes: Vec<Vec<GroundState>> = Vec::new();

        for _ in 0..lines[0].len() {
            stripes.push(Vec::new());
        }

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                stripes[i].push(match c {
                    '.' => GroundState::Dirt,
                    '#' => GroundState::SquareRock,
                    'O' => GroundState::RoundRock,
                    _ => panic!("Bad ground type"),
                });
            }
        }

        Self {
            grid: Array2D::from_columns(&stripes).unwrap(),
        }
    }

    pub fn print(&self) {
        println!("<------");
        for row in self.grid.rows_iter() {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!("<------");
    }

    pub fn slide_all_rocks_north(&mut self) {
        //Walk down each column, and slide rocks up

        for column in 0..self.grid.num_columns() {
            let mut dirt_start = 0xFFFF;

            for row in 0..self.grid.num_rows() {
                match self.grid[(row, column)] {
                    GroundState::SquareRock => {
                        //Cant be moved, so have to flush queued dirt then insert
                        if dirt_start != 0xFFFF {
                            for x in dirt_start..row {
                                self.grid[(x, column)] = GroundState::Dirt;
                            }
                            dirt_start = 0xFFFF;
                        }
                    }
                    GroundState::RoundRock => {
                        if dirt_start == 0xFFFF {
                            //Cant move, leave in place
                        } else {
                            self.grid[(dirt_start, column)] = GroundState::RoundRock;
                            dirt_start += 1;
                        }
                    }
                    GroundState::Dirt => {
                        if dirt_start == 0xFFFF {
                            dirt_start = row;
                        }
                    }
                }
            }
            if dirt_start != 0xFFFF {
                for x in dirt_start..self.grid.num_rows() {
                    self.grid[(x, column)] = GroundState::Dirt;
                }
            }
        }
    }

    pub fn slide_all_rocks_east(&mut self) {
        //Walk across the row, sliding right
        for row in 0..self.grid.num_rows() {
            let mut dirt_start = 0xFFFF;

            for column in (0..self.grid.num_columns()).rev() {
                match self.grid[(row, column)] {
                    GroundState::SquareRock => {
                        //Cant be moved, so have to flush queued dirt then insert
                        if dirt_start != 0xFFFF {
                            for x in column..dirt_start {
                                self.grid[(row, x + 1)] = GroundState::Dirt;
                            }
                            dirt_start = 0xFFFF;
                        }
                    }
                    GroundState::RoundRock => {
                        if dirt_start == 0xFFFF {
                            //Cant move, leave in place
                        } else {
                            self.grid[(row, dirt_start)] = GroundState::RoundRock;
                            dirt_start -= 1;
                        }
                    }
                    GroundState::Dirt => {
                        if dirt_start == 0xFFFF {
                            dirt_start = column;
                        }
                    }
                }
            }
            if dirt_start != 0xFFFF {
                for x in 0..dirt_start + 1 {
                    self.grid[(row, x)] = GroundState::Dirt;
                }
            }
        }
    }

    pub fn slide_all_rocks_west(&mut self) {
        //Walk across the row, sliding left
        for row in 0..self.grid.num_rows() {
            let mut dirt_start = 0xFFFF;

            for column in 0..self.grid.num_columns() {
                match self.grid[(row, column)] {
                    GroundState::SquareRock => {
                        //Cant be moved, so have to flush queued dirt then insert
                        if dirt_start != 0xFFFF {
                            for x in dirt_start..column {
                                self.grid[(row, x)] = GroundState::Dirt;
                            }
                            dirt_start = 0xFFFF;
                        }
                    }
                    GroundState::RoundRock => {
                        if dirt_start == 0xFFFF {
                            //Cant move, leave in place
                        } else {
                            self.grid[(row, dirt_start)] = GroundState::RoundRock;
                            dirt_start += 1;
                        }
                    }
                    GroundState::Dirt => {
                        if dirt_start == 0xFFFF {
                            dirt_start = column;
                        }
                    }
                }
            }
            if dirt_start != 0xFFFF {
                for x in dirt_start..self.grid.num_columns() {
                    self.grid[(row, x)] = GroundState::Dirt;
                }
            }
        }
    }
    pub fn slide_all_rocks_south(&mut self) {
        //Walk down each column, and slide rocks down
        for column in 0..self.grid.num_columns() {
            let mut dirt_start = 0xFFFF;

            for row in (0..self.grid.num_rows()).rev() {
                match self.grid[(row, column)] {
                    GroundState::SquareRock => {
                        //Cant be moved, so have to flush queued dirt then insert
                        if dirt_start != 0xFFFF {
                            for x in row..dirt_start {
                                self.grid[(x + 1, column)] = GroundState::Dirt;
                            }
                            dirt_start = 0xFFFF;
                        }
                    }
                    GroundState::RoundRock => {
                        if dirt_start == 0xFFFF {
                            //Cant move, leave in place
                        } else {
                            self.grid[(dirt_start, column)] = GroundState::RoundRock;
                            dirt_start -= 1;
                        }
                    }
                    GroundState::Dirt => {
                        if dirt_start == 0xFFFF {
                            dirt_start = row;
                        }
                    }
                }
            }
            if dirt_start != 0xFFFF {
                for x in (0..dirt_start + 1).rev() {
                    self.grid[(x, column)] = GroundState::Dirt;
                }
            }
        }
    }

    pub fn get_north_weight(&self) -> usize {
        let mut sum = 0;
        for (x, row) in self.grid.rows_iter().enumerate() {
            for v in row {
                if *v == GroundState::RoundRock {
                    sum += self.grid.num_rows() - x;
                }
            }
        }
        sum
    }
}
#[memoize(Capacity: 1_000_000)]

fn rotate_one_iter(mut grid: Grid) -> Grid {
    grid.slide_all_rocks_north();

    grid.slide_all_rocks_west();

    grid.slide_all_rocks_south();

    grid.slide_all_rocks_east();
    grid
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut grid: Grid = Grid::from_lines(&lines);
    let mut last_iter = Instant::now();
    grid.print();
    for i in 0..1_000_000_000 {
        grid = rotate_one_iter(grid);
        if i % 1_000_000 == 0 {
            println!(
                "i {} @ {} ms",
                i,
                Instant::now().duration_since(last_iter).as_millis()
            );
            last_iter = Instant::now();
        }
    }
    grid.get_north_weight()
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
