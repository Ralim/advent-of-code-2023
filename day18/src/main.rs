use array2d::Array2D;
use std::fs::read_to_string;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::North,
            "D" => Self::South,
            "L" => Self::West,
            "R" => Self::East,
            _ => panic!("Bad Direction"),
        }
    }
    pub fn move_cursor(&self, po: (usize, usize)) -> (usize, usize) {
        // println!("dir {:?}, pos {:?}", self, po);
        match self {
            Direction::North => (po.0 - 1, po.1),
            Direction::East => (po.0, po.1 + 1),
            Direction::South => (po.0 + 1, po.1),
            Direction::West => (po.0, po.1 - 1),
        }
    }
}
#[derive(Clone, PartialEq, Hash, Eq)]
struct Command {
    dir: Direction,
    dist: i64,
    colour: String,
}
impl Command {
    pub fn from_line(line: &str) -> Self {
        //D 2 (#411b91)
        let parts: Vec<&str> = line.split(' ').collect();
        Self {
            dir: Direction::from_str(parts[0]),
            dist: parts[1].parse().unwrap(),
            colour: parts[2].to_owned(),
        }
    }
    pub fn convert_part_2(&self) -> Self {
        let distance_str = &self.colour[2..7];
        let dir = match &self.colour.chars().collect::<Vec<char>>()[7] {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => unreachable!(),
        };
        println!("Col {} => {}", self.colour, distance_str);

        Self {
            dir: dir,
            dist: i64::from_str_radix(&distance_str, 16).unwrap(),
            colour: self.colour.clone(),
        }
    }
}
#[derive(Clone, PartialEq, Hash, Eq)]
struct DigCell {
    dug: bool,
    colour: String,
}

fn commands_to_area(commands: &[Command]) -> i64 {
    let (mut a, mut r, mut c) = (0, 0, 0);

    for cmd in commands {
        let (rr, cc) = (r, c);
        match cmd.dir {
            Direction::North => r -= cmd.dist,
            Direction::East => c += cmd.dist,
            Direction::South => r += cmd.dist,
            Direction::West => c -= cmd.dist,
            _ => unreachable!(),
        };

        a += (c + cc) * (r - rr) + cmd.dist; // shoestring loop
    }
    a / 2 + 1
}

fn main() {
    println!("Loading...");
    let file_contents = read_to_string("input").unwrap();
    let actions: Vec<Command> = file_contents
        .lines()
        .map(|line| Command::from_line(line))
        .collect();

    let area = commands_to_area(&actions);
    println!("Part 1 {}", area);
    let part_2_cmds: Vec<Command> = actions.iter().map(|x| x.convert_part_2()).collect();
    let area2 = commands_to_area(&part_2_cmds);
    println!("Part 2 {}", area2);
}
