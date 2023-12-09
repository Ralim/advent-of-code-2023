use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unhandled dir"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StepMap {
    pub left_step: String,
    pub right_step: String,
}

impl StepMap {
    pub fn make(value: &str) -> Self {
        let mut cleaned = value.replace('(', "");
        cleaned = cleaned.replace(')', "");
        let chars: Vec<&str> = cleaned.split(',').collect();
        let left = chars[0].trim();
        let right = chars[1].trim();

        Self {
            left_step: left.to_owned(),
            right_step: right.to_owned(),
        }
    }
    pub fn get_value(&self, step: &Direction) -> &str {
        match step {
            Direction::Left => &self.left_step,
            Direction::Right => &self.right_step,
        }
    }
}

fn find_steps_until_terminated_z(
    steps: &HashMap<&str, StepMap>,
    instructions: &[Direction],
    start: &str,
) -> usize {
    let mut step_count = 0;
    let mut cur = start;
    for dir in instructions.iter().cycle() {
        cur = steps[cur].get_value(dir);
        step_count += 1;
        if cur.ends_with("Z") {
            break;
        }
    }
    step_count
}

fn read_file(filename: &str) -> usize {
    let mut steps: HashMap<&str, StepMap> = HashMap::new();

    let file_contents = read_to_string(filename).unwrap();
    let mut lines: Vec<&str> = file_contents.lines().collect();
    let mut instructions = lines.remove(0);
    instructions = instructions.trim();
    let instructions_lookup: Vec<Direction> = instructions
        .chars()
        .map(|c| Direction::from_char(c))
        .collect();

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let sp: Vec<&str> = line.split('=').collect();
        let key = sp[0].trim();
        let value = sp[1].trim();
        let hand = StepMap::make(&value);
        steps.insert(key, hand);
    }

    let starting_points: Vec<&str> = steps
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|v| v.to_owned())
        .collect();
    let mut minimal_loops: HashSet<usize> = HashSet::new();
    for start in starting_points {
        let smallest_loop = find_steps_until_terminated_z(&steps, &instructions_lookup, start);
        println!("Start -> Loop: {} -> {}", start, smallest_loop);
        minimal_loops.insert(smallest_loop);
    }
    let final_loops: Vec<usize> = minimal_loops.iter().map(|f| *f).collect();
    // Each loop is the minimal number of steps required to go from "start" -> A valid endpoint.
    // Once you have rolled off the end of the loop, its going to throw you to _somewhere_ else to start a new loop
    // If we assume that the input shall hit an end target loop, and then continue to re-loop
    // We are looking for all of these smaller loops to align
    // Alignments occur at the first lined up common multiple
    // We want the lower one
    lowest_common_multiple_of_set(&final_loops)
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}

pub fn lowest_common_multiple_of_set(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lowest_common_multiple_of_set(&nums[1..]);
    a * b / greatest_common_mult_pair(a, b)
}

fn greatest_common_mult_pair(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    greatest_common_mult_pair(b, a % b)
}
