use array2d::Array2D;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

#[derive(Clone, PartialEq, Hash, Eq)]
struct Grid {
    heat_loss_grid: Array2D<usize>,
}

impl Grid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut stripes: Vec<Vec<usize>> = Vec::new();

        for _ in 0..lines[0].len() {
            stripes.push(Vec::new());
        }

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                stripes[i].push(c.to_string().parse().unwrap());
            }
        }

        Self {
            heat_loss_grid: Array2D::from_columns(&stripes).unwrap(),
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
    }

    fn dijkstra(
        &self,
        minimum_movement_per_iter: isize,
        maxium_movement_in_a_line: isize,
    ) -> isize {
        let mut distances = HashMap::new();

        let mut q = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

        while let Some((cost, (r, c, d))) = q.pop() {
            if (r, c)
                == (
                    self.heat_loss_grid.row_len() - 1,
                    self.heat_loss_grid.column_len() - 1,
                )
            {
                return -cost;
            }
            if distances.get(&(r, c, d)).is_some_and(|&c| -cost > c) {
                continue;
            }
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if d == (dr, dc) || d == (-dr, -dc) {
                    continue;
                }
                let mut next_cost = -cost;

                for dist in 1..maxium_movement_in_a_line + 1 {
                    let rr = (r as isize + dr * dist) as usize;
                    let cc = (c as isize + dc * dist) as usize;
                    // If we leave the grid, yeet
                    if rr >= self.heat_loss_grid.row_len() || cc >= self.heat_loss_grid.column_len()
                    {
                        continue;
                    }
                    next_cost += (self.heat_loss_grid[(rr, cc)]) as isize;
                    if dist < minimum_movement_per_iter {
                        continue;
                    }
                    let key = (rr, cc, (dr, dc));
                    if next_cost < *distances.get(&key).unwrap_or(&isize::MAX) {
                        distances.insert(key, next_cost);
                        q.push((-next_cost, key));
                    }
                }
            }
        }
        panic!()
    }
}

fn read_file(filename: &str) -> isize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut base_grid: Grid = Grid::from_lines(&lines);

    base_grid.print();

    base_grid.dijkstra(4, 10)
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
