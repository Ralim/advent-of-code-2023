use array2d::Array2D;

use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn get_new_directions(&self) -> Vec<Direction> {
        //
        match self {
            Direction::North => vec![Direction::North, Direction::East, Direction::West],
            Direction::South => vec![Direction::East, Direction::South, Direction::West],
            Direction::East => vec![Direction::North, Direction::East, Direction::South],
            Direction::West => vec![Direction::North, Direction::South, Direction::West],
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Grid {
    heat_loss_grid: Array2D<usize>,
    summed_cost_grid: Array2D<usize>,
}

impl Grid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut stripes: Vec<Vec<usize>> = Vec::new();
        let mut activated_tiles: Vec<Vec<usize>> = Vec::new();

        for _ in 0..lines[0].len() {
            stripes.push(Vec::new());
            activated_tiles.push(Vec::new());
        }

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                stripes[i].push(c.to_string().parse().unwrap());
                activated_tiles[i].push(0xFFFFFFFF);
            }
        }

        Self {
            heat_loss_grid: Array2D::from_columns(&stripes).unwrap(),
            summed_cost_grid: Array2D::from_columns(&activated_tiles).unwrap(),
        }
    }

    pub fn print(&self) {
        println!("<------");
        for row in self.heat_loss_grid.rows_iter() {
            for c in row {
                print!("{:03},", c);
            }
            println!();
        }
        println!("<------");
        println!("-------->");
        for row in self.summed_cost_grid.rows_iter() {
            for c in row {
                print!("{:03},", c);
            }
            println!();
        }
        println!("-------->");
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
                && row < self.heat_loss_grid.num_rows() as i64
                && col >= 0
                && col < self.heat_loss_grid.num_columns() as i64,
        )
    }

    fn create_cost_grid(
        &mut self,
        current_row: i64,
        current_col: i64,
        current_direction: Direction,
        mut current_cost: usize,
        dir1: Direction,
        dir2: Direction,
    ) {
        // Starting at the given exit, we create a grid of accumulated costs, working backwards.
        // The value at each point is the sum of its cost + the lowest value from it.
        // this can be thought of each point containing the estimated burn-down of the cost to the exit

        current_cost += self.heat_loss_grid[(current_row as usize, current_col as usize)];
        let existing_cost_value =
            self.summed_cost_grid[(current_row as usize, current_col as usize)];
        if current_cost >= existing_cost_value {
            return;
        }
        self.summed_cost_grid[(current_row as usize, current_col as usize)] = current_cost;

        let banned_direction = if dir1 == dir2 { Some(dir1) } else { None };

        for direction in current_direction.get_new_directions() {
            if banned_direction == Some(direction) {
                continue; // skip if 3 in a row
            }

            let (new_row, new_col, in_grid) =
                self.get_new_point(direction, current_row, current_col);

            if in_grid {
                self.create_cost_grid(new_row, new_col, direction, current_cost, direction, dir1);
            }
        }
    }

    pub fn walk_decent_path_sum_heat_loss(
        &self,

        current_row: i64,
        current_col: i64,
        current_direction: Direction,
        mut current_sum: usize,
        history: &mut HashSet<(usize, usize, Direction)>,
        dir1: Direction,
        dir2: Direction,
    ) -> usize {
        let pos = (current_row as usize, current_col as usize);
        // println!("Pos {:?} -> {}", pos, current_sum);
        //If we are at the target
        if current_row as usize == self.heat_loss_grid.num_rows() - 1
            && current_col as usize == self.heat_loss_grid.num_columns() - 1
        {
            return current_sum;
        }
        if history.contains(&(
            current_row as usize,
            current_col as usize,
            current_direction,
        )) {
            return 0xFFFFFFFF;
        }
        history.insert((
            current_row as usize,
            current_col as usize,
            current_direction,
        ));

        //Not there yet
        current_sum += self.heat_loss_grid[pos] as usize;

        let directions = current_direction.get_new_directions();
        let mut costs: Vec<usize> = Vec::with_capacity(3);
        //TODO filter based on max-three-in-a-row rule
        let banned_direction = if dir1 == dir2 { Some(dir1) } else { None };

        // for direction in &directions {
        //     if banned_direction == Some(*direction) {
        //         continue; // skip if 3 in a row
        //     }
        //     let (new_row, new_col, in_grid) =
        //         self.get_new_point(*direction, current_row, current_col);

        //     if in_grid {
        //         let mut dirs = direction_history.clone();
        //         dirs.push(*direction);
        //         if dirs.len() >= 3 {
        //             let _ = dirs.pop();
        //         }

        //         costs.push(self.walk_decent_path_sum_heat_loss(
        //             new_row,
        //             new_col,
        //             *direction,
        //             current_sum,
        //             history,
        //             dirs,
        //         ));
        //     } else {
        //         costs.push(0xFFFFFFFF);
        //     }
        // }
        // *costs.iter().min().unwrap()

        for direction in &directions {
            let (new_row, new_col, in_grid) =
                self.get_new_point(*direction, current_row, current_col);

            if in_grid
            /*and valid*/
            {
                costs.push(self.heat_loss_grid[(new_row as usize, new_col as usize)]);
            } else {
                costs.push(0xFFFFFFFF);
            }
        }

        let min_step = *costs.iter().min().unwrap();
        let direction = directions[costs.iter().position(|x| *x == min_step).unwrap()];

        let (new_row, new_col, in_grid) = self.get_new_point(direction, current_row, current_col);
        if !in_grid {
            panic!("Uh oh");
        }
        self.walk_decent_path_sum_heat_loss(
            new_row,
            new_col,
            direction,
            current_sum,
            history,
            direction,
            dir1,
        )
    }
    pub fn hunt_lowest_cost_path_to_exit(&mut self, current_row: i64, current_col: i64) -> usize {
        //Starting in the top left, follow the beam right
        let mut history = HashSet::new();
        self.walk_decent_path_sum_heat_loss(
            current_row,
            current_col,
            Direction::South,
            0,
            &mut history,
            Direction::South,
            Direction::South,
        )
    }
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut base_grid: Grid = Grid::from_lines(&lines);
    // let mut last_iter = Instant::now();

    base_grid.create_cost_grid(
        (base_grid.heat_loss_grid.num_rows() - 1) as i64,
        (base_grid.heat_loss_grid.num_columns() - 1) as i64,
        Direction::North,
        0,
        Direction::South,
        Direction::South,
    );

    base_grid.print();

    // base_grid.hunt_lowest_cost_path_to_exit(0, 0)
    0
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
