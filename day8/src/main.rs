use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    fs::read_to_string,
};

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
    pub fn get_value(&self, step_right: bool) -> &str {
        return if step_right {
            &self.right_step
        } else {
            &self.left_step
        };
    }
}

fn read_file(filename: &str) -> u64 {
    let mut steps: BTreeMap<&str, StepMap> = BTreeMap::new();

    let file_contents = read_to_string(filename).unwrap();
    let mut lines: Vec<&str> = file_contents.lines().collect();
    let mut instructions = lines.remove(0);
    instructions = instructions.trim();
    let instructions_lookup: Vec<char> = instructions.chars().collect();
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let sp: Vec<&str> = line.split('=').collect();
        println!("Line {} -> {:?}", line, sp);
        let key = sp[0].trim();
        let value = sp[1].trim();

        let hand = StepMap::make(&value);

        steps.insert(key, hand);
    }
    let mut current_step = "AAA";
    let final_step = "ZZZ";
    let mut index = 0;
    while current_step != final_step {
        let going_right = instructions_lookup[index % instructions_lookup.len()] == 'R';
        let new_step = steps[current_step].get_value(going_right);
        println!(
            "Step {} -> {} (right{})",
            current_step, new_step, going_right
        );
        current_step = new_step;
        index += 1;
    }
    index as u64
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
