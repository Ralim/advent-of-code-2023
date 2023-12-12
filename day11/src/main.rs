use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPoint {
    pub row: isize,
    pub col: isize,
}

impl GridPoint {
    //Walk the line distance with no diagonals
    pub fn distance(&self, b: &GridPoint, num_col_gap: usize, num_row_gap: usize) -> isize {
        let mut row_error: isize = isize::abs((b.row) - (self.row));
        let mut col_error: isize = isize::abs((b.col) - (self.col));

        row_error += (num_row_gap * (1000000 - 1)) as isize;
        col_error += (num_col_gap * (1000000 - 1)) as isize;

        isize::abs(row_error) + isize::abs(col_error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Galaxy {
    location: GridPoint,
}

impl Galaxy {
    //
}
struct PipeGrid {
    galaxies: Vec<Galaxy>,
    total_cols: isize,
    total_rows: isize,
}

impl PipeGrid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut total_rows = 0;
        let mut total_cols = 0;

        for (row, line) in lines.iter().enumerate() {
            total_cols = line.len() as isize;
            total_rows += 1;
            for (col, c) in line.chars().enumerate() {
                //
                if c == '.' {
                    //Space gap
                } else {
                    galaxies.push(Galaxy {
                        location: GridPoint {
                            row: row as isize,
                            col: col as isize,
                        },
                    });
                }
            }
        }

        Self {
            galaxies,
            total_cols,
            total_rows,
        }
    }
    fn get_empty_rows(&self) -> Vec<isize> {
        let mut empty_rows: HashSet<isize> = HashSet::new();
        empty_rows.extend(0..self.total_rows);
        // Walk all galaxies, annex any row & col contained
        for gal in &self.galaxies {
            empty_rows.remove(&gal.location.row);
        }
        let sorted_empty_rows: Vec<isize> = empty_rows.iter().copied().collect();
        sorted_empty_rows
    }
    fn get_empty_cols(&self) -> Vec<isize> {
        let mut empty_cols: HashSet<isize> = HashSet::new();
        empty_cols.extend(0..self.total_cols);
        // Walk all galaxies, annex any row & col contained
        for gal in &self.galaxies {
            empty_cols.remove(&gal.location.col);
        }

        let sorted_empty_cols: Vec<isize> = empty_cols.iter().copied().collect();
        sorted_empty_cols
    }

    fn get_total_distance_galaxy_pairs(&self) -> isize {
        let mut total_len = 0;
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();
        println!("Empty rows {:?}", empty_rows);
        println!("Empty cols {:?}", empty_cols);

        for (i, ref_galaxy) in self.galaxies.iter().enumerate() {
            for pair_galaxy_index in (i + 1)..self.galaxies.len() {
                let compare_galaxy = &self.galaxies[pair_galaxy_index].location;

                let row_start = compare_galaxy.row.min(ref_galaxy.location.row);
                let row_end = compare_galaxy.row.max(ref_galaxy.location.row);
                let num_gap_rows = empty_rows
                    .iter()
                    .filter(|r| **r >= row_start && **r <= row_end)
                    .count();

                let col_start = compare_galaxy.col.min(ref_galaxy.location.col);
                let col_end = compare_galaxy.col.max(ref_galaxy.location.col);
                let num_gap_cols = empty_cols
                    .iter()
                    .filter(|r| **r >= col_start && **r <= col_end)
                    .count();

                let len = ref_galaxy
                    .location
                    .distance(compare_galaxy, num_gap_cols, num_gap_rows);
                println!(
                    "Distance {:?} -> {:?} = {}; row gap {}, col gap {}",
                    ref_galaxy, compare_galaxy, len, num_gap_rows, num_gap_cols
                );
                total_len += len;
            }
        }
        total_len
    }
}

fn read_file(filename: &str) -> isize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let grid = PipeGrid::from_lines(&lines);
    grid.get_total_distance_galaxy_pairs()
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
