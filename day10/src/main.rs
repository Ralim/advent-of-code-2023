use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeDirection {
    North,
    East,
    South,
    West,
}

impl PipeDirection {
    pub fn move_cursor(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            PipeDirection::North => (x - 1, y),
            PipeDirection::East => (x, y + 1),
            PipeDirection::South => (x + 1, y),
            PipeDirection::West => (x, y - 1),
        }
    }
    pub fn flip(&self) -> Self {
        match self {
            PipeDirection::North => PipeDirection::South,
            PipeDirection::East => PipeDirection::West,
            PipeDirection::South => PipeDirection::North,
            PipeDirection::West => PipeDirection::East,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PipeCell {
    pub directions: [PipeDirection; 2],
}

impl PipeCell {
    pub fn from_char(value: &char) -> Self {
        let directions = match value {
            '|' => [PipeDirection::North, PipeDirection::South],
            '-' => [PipeDirection::East, PipeDirection::West],
            'L' => [PipeDirection::North, PipeDirection::East],
            'J' => [PipeDirection::West, PipeDirection::North],
            '7' => [PipeDirection::West, PipeDirection::South],
            'F' => [PipeDirection::East, PipeDirection::South],
            _ => panic!("Invalid pipe {}", value),
        };
        Self { directions }
    }
    pub fn from_dirs(directions: &[PipeDirection]) -> Self {
        Self {
            directions: [directions[0], directions[1]],
        }
    }
    pub fn connects(&self, dir: PipeDirection) -> bool {
        dir == self.directions[0] || dir == self.directions[1]
    }
    pub fn get_next_dir(&self, input: PipeDirection) -> PipeDirection {
        if input == self.directions[0] {
            self.directions[1]
        } else {
            self.directions[0]
        }
    }
}
struct PipeGrid {
    grid: Vec<Vec<Option<PipeCell>>>,
    start_x: usize,
    start_y: usize,
}

impl PipeGrid {
    pub fn from_lines(lines: &[&str]) -> Self {
        //
        let mut pipe_row: Vec<Option<PipeCell>> = Vec::new();
        let mut grid: Vec<Vec<Option<PipeCell>>> = Vec::new();
        let mut cursor_x = 0;
        let mut cursor_y = 0;

        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                //
                if c == '.' {
                    pipe_row.push(None);
                } else if c == 'S' {
                    //Start pos
                    cursor_x = row;
                    cursor_y = col;
                    pipe_row.push(None);
                } else {
                    pipe_row.push(Some(PipeCell::from_char(&c)));
                }
            }
            grid.push(pipe_row.clone());
            pipe_row.clear();
        }
        //Have to now go back and re-write the start point so we know its directions
        let mut start_dirs: Vec<PipeDirection> = Vec::new();

        if cursor_x > 0 {
            //
            if grid[cursor_x - 1][cursor_y].is_some_and(|p| p.connects(PipeDirection::South)) {
                start_dirs.push(PipeDirection::North);
            }
        }
        if cursor_y > 0 {
            //
            if grid[cursor_x][cursor_y - 1].is_some_and(|p| p.connects(PipeDirection::East)) {
                start_dirs.push(PipeDirection::West);
            }
        }
        if cursor_x < grid.len() {
            //
            if grid[cursor_x + 1][cursor_y].is_some_and(|p| p.connects(PipeDirection::North)) {
                start_dirs.push(PipeDirection::South);
            }
        }
        if cursor_y < grid[0].len() {
            //
            if grid[cursor_x][cursor_y + 1].is_some_and(|p| p.connects(PipeDirection::West)) {
                start_dirs.push(PipeDirection::East);
            }
        }
        if start_dirs.len() != 2 {
            panic!("Bad start decode, start @ {}/{}", cursor_x, cursor_y);
        } else {
            println!(
                "Start decoded as {:?} @ {}/{}",
                start_dirs, cursor_x, cursor_y
            );
        }
        grid[cursor_x][cursor_y] = Some(PipeCell::from_dirs(&start_dirs));

        Self {
            grid,
            start_x: cursor_x,
            start_y: cursor_y,
        }
    }
    pub fn get_loop_len(&mut self) -> usize {
        //Starting at S, follow pipes until we get to the start

        let mut cursor_x = self.start_x;
        let mut cursor_y = self.start_y;

        let mut direction = self.grid[cursor_x][cursor_y].unwrap().directions[0];
        (cursor_x, cursor_y) = direction.move_cursor(cursor_x, cursor_y);

        let mut steps = 1;
        while cursor_x != self.start_x || cursor_y != self.start_y {
            let cell = self.grid[cursor_x][cursor_y];
            println!(
                "Step {},{} -> {:?} -> {:?}",
                cursor_x, cursor_y, direction, cell
            );
            direction = cell.unwrap().get_next_dir(direction.flip());
            (cursor_x, cursor_y) = direction.move_cursor(cursor_x, cursor_y);
            steps += 1;
        }
        steps
    }
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut grid = PipeGrid::from_lines(&lines);
    println!("Grid {:?}", grid.grid);
    grid.get_loop_len() / 2
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
