use array2d::Array2D;
use memoize::memoize;

use std::{
    collections::HashSet,
    fmt::{self},
    fs::read_to_string,
    time::Instant,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Optic {
    Dirt,
    MirrorNESW,
    MirrorNWSE,
    HorizontalSplitter,
    VerticalSplitter,
}
impl fmt::Display for Optic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Optic::MirrorNESW => write!(f, "/"),
            Optic::MirrorNWSE => write!(f, "\\"),
            Optic::HorizontalSplitter => write!(f, "-"),
            Optic::VerticalSplitter => write!(f, "|"),
            Optic::Dirt => write!(f, "."),
        }
    }
}

impl Optic {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Optic::Dirt,
            '\\' => Optic::MirrorNWSE,
            '/' => Optic::MirrorNESW,
            '-' => Optic::HorizontalSplitter,
            '|' => Optic::VerticalSplitter,
            _ => panic!("Bad ground type"),
        }
    }
    pub fn get_new_directions(&self, dir: Direction) -> Vec<Direction> {
        //
        match self {
            Optic::Dirt => {
                //
                vec![dir]
            }
            Optic::MirrorNESW => match dir {
                // /
                Direction::North => vec![Direction::East],
                Direction::South => vec![Direction::West],
                Direction::East => vec![Direction::North],
                Direction::West => vec![Direction::South],
            },
            Optic::MirrorNWSE => match dir {
                // \
                Direction::North => vec![Direction::West],
                Direction::South => vec![Direction::East],
                Direction::East => vec![Direction::South],
                Direction::West => vec![Direction::North],
            },
            Optic::HorizontalSplitter => match dir {
                Direction::North => vec![Direction::East, Direction::West],
                Direction::South => vec![Direction::East, Direction::West],
                Direction::East => vec![dir],
                Direction::West => vec![dir],
            },
            Optic::VerticalSplitter => match dir {
                Direction::East => vec![Direction::North, Direction::South],
                Direction::West => vec![Direction::North, Direction::South],
                Direction::North => vec![dir],
                Direction::South => vec![dir],
            },
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Grid {
    grid: Array2D<Optic>,
    activated_tiles: Array2D<i64>,
}

impl Grid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut stripes: Vec<Vec<Optic>> = Vec::new();
        let mut activated_tiles: Vec<Vec<i64>> = Vec::new();

        for _ in 0..lines[0].len() {
            stripes.push(Vec::new());
            activated_tiles.push(Vec::new());
        }

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                stripes[i].push(Optic::from_char(c));
                activated_tiles[i].push(0);
            }
        }

        Self {
            grid: Array2D::from_columns(&stripes).unwrap(),
            activated_tiles: Array2D::from_columns(&activated_tiles).unwrap(),
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
    pub fn get_count_activated_cells(&self) -> i64 {
        self.activated_tiles
            .elements_row_major_iter()
            .map(|e| if *e > 0 { 1 } else { 0 })
            .sum()
    }
    fn get_new_point(&self, dir: Direction, mut row: i64, mut col: i64) -> (i64, i64, bool) {
        match dir {
            Direction::North => row -= 1,
            Direction::South => row += 1,
            Direction::East => col += 1,
            Direction::West => col -= 1,
        }
        (
            row,
            col,
            row >= 0
                && row < self.grid.num_rows() as i64
                && col >= 0
                && col < self.grid.num_columns() as i64,
        )
    }
    fn follow_beam_update_counter(
        &mut self,
        current_row: i64,
        current_col: i64,
        current_direction: Direction,
        history: &mut HashSet<(usize, usize, Direction)>,
    ) {
        let pos = (current_row as usize, current_col as usize);
        let history_key = (
            current_row as usize,
            current_col as usize,
            current_direction,
        );
        self.activated_tiles[pos] += 1;
        if history.contains(&history_key) {
            return;
        }
        history.insert(history_key);

        let new_directions = self.grid[pos].get_new_directions(current_direction);

        for direction in new_directions {
            let (new_row, new_col, in_grid) =
                self.get_new_point(direction, current_row, current_col);
            if in_grid {
                self.follow_beam_update_counter(new_row, new_col, direction, history);
            }
        }
    }
    pub fn update_tile_activation(&mut self) {
        //Starting in the top left, follow the beam right
        let mut history = HashSet::new();
        self.follow_beam_update_counter(0, 0, Direction::East, &mut history);
    }
}

// #[memoize(Capacity: 1_000_000)]

// fn rotate_one_iter(mut grid: Grid) -> Grid {
//     grid.slide_all_rocks_north();

//     grid.slide_all_rocks_west();

//     grid.slide_all_rocks_south();

//     grid.slide_all_rocks_east();
//     grid
// }

fn read_file(filename: &str) -> i64 {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut grid: Grid = Grid::from_lines(&lines);
    // let mut last_iter = Instant::now();

    grid.print();
    grid.update_tile_activation(); // fills out the active grid
    grid.get_count_activated_cells()
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
