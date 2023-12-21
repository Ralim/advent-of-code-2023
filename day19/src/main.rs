use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum Action {
    GreaterThan,
    LessThan,
    NoOp,
}
impl Action {
    pub fn from_char(s: &char) -> Self {
        match s {
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            _ => panic!("Bad Actionn"),
        }
    }
    pub fn action(&self, a: usize, b: usize) -> bool {
        match self {
            Action::GreaterThan => a > b,
            Action::LessThan => a < b,
            Action::NoOp => true,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    pub fn from_line(line: &str) -> Self {
        //D 2 (#411b91)
        let binding = line
            .replace("{", "")
            .replace("}", "")
            .replace("x=", "")
            .replace("m=", "")
            .replace("a=", "")
            .replace("s=", "");
        let parts: Vec<&str> = binding.split(',').collect();
        println!("x {:?}", parts);
        Self {
            x: parts[0].parse().unwrap(),
            m: parts[1].parse().unwrap(),
            a: parts[2].parse().unwrap(),
            s: parts[3].parse().unwrap(),
        }
    }
    pub fn get_value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Instruction {
    pub output_chain: String,
    attribute: char,
    action: Action,
    threshold: usize,
}
impl Instruction {
    pub fn from_str(ll: &str) -> Self {
        if !ll.contains(":") {
            return Self {
                output_chain: ll.to_owned(),
                attribute: '!',
                action: Action::NoOp,
                threshold: 0,
            };
        }
        //Okay we have the colon so it requires an actual action
        let chars: Vec<char> = ll.chars().collect();
        let splits = ll.split(":").collect::<Vec<&str>>();
        let output_chain = splits[1];
        Self {
            output_chain: output_chain.to_owned(),
            attribute: chars[0],
            action: Action::from_char(&chars[1]),
            threshold: (splits[0][2..]).parse().unwrap(),
        }
    }
    pub fn test(&self, part: &Part) -> bool {
        let value = match self.attribute {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            '!' => 0,
            _ => unreachable!(),
        };
        match self.action {
            Action::GreaterThan => value > self.threshold,
            Action::LessThan => value < self.threshold,
            Action::NoOp => true,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct InstructionChain {
    name: String,
    tests: Vec<Instruction>,
}
impl InstructionChain {
    pub fn from_line(ll: &str) -> Self {
        // qqz{s>2770:qs,m<1801:hdj,R}
        let name_split: Vec<&str> = ll.split("{").collect();
        let instructions = name_split[1]
            .replace("}", "")
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|z| Instruction::from_str(*z))
            .collect();

        Self {
            name: name_split[0].to_owned(),
            tests: instructions,
        }
    }
    pub fn match_to_output(&self, part: &Part) -> String {
        for test in &self.tests {
            if test.test(part) {
                return test.output_chain.clone();
            }
        }
        unreachable!();
    }
}
fn find_next_chain(
    part: &Part,
    instructions: &HashMap<String, InstructionChain>,
    chain: &str,
) -> String {
    let instruction = &instructions[chain];
    instruction.match_to_output(part)
}
fn sort_part_to_accepted_or_not(
    part: &Part,
    instructions: &HashMap<String, InstructionChain>,
) -> bool {
    let mut chain = "in".to_owned();
    loop {
        let new_chain = find_next_chain(part, instructions, &chain);
        // println!("Chaining from {} to {}", chain, new_chain);
        chain = new_chain;
        if chain == "A" {
            return true;
        }
        if chain == "R" {
            return false;
        }
    }
}

fn count_allowed_range(
    instructions: &HashMap<String, InstructionChain>,
    chain: &str,
    mut ranges: [Vec<usize>; 4],
) -> usize {
    if chain == "A" {
        return ranges.iter().map(|v| v.len()).product();
    }
    if chain == "R" {
        return 0;
    }

    //Otherwise recurse and narrow

    let mut sum = 0;
    //Recurse down all optional paths
    let instruction = &instructions[chain];
    for comparison in &instruction.tests {
        if comparison.action == Action::NoOp {
            //This is the failsafe last one, recurse back down
            sum += count_allowed_range(instructions, &comparison.output_chain, ranges.clone());
            continue;
        }
        let array_slot = match comparison.attribute {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        //Create a cropped in range
        let mut windowed_range = ranges.clone(); // New copy thats safe to poke

        //Window in the param we care about
        let less_than = comparison.action == Action::LessThan;
        //Here we split the range, so that which passes goes into recursive new window, that which doesnt is updated in our current range set, so that later checks can test it
        (windowed_range[array_slot], ranges[array_slot]) =
            ranges[array_slot].iter().partition(|&&val| {
                if less_than {
                    val < comparison.threshold
                } else {
                    val > comparison.threshold
                }
            });
        sum += count_allowed_range(instructions, &comparison.output_chain, windowed_range);
    }
    sum
}
fn main() {
    println!("Loading...");
    let file_contents = read_to_string("input").unwrap();
    let mut split_seen = false;
    let mut chains = HashMap::new();
    let mut objects = Vec::new();
    for line in file_contents.lines() {
        if line.trim().len() < 1 {
            split_seen = true;
        } else {
            if split_seen {
                let obj = Part::from_line(line);
                objects.push(obj);
            } else {
                let instruction_chain = InstructionChain::from_line(line);
                let name = instruction_chain.name.clone();
                chains.insert(name, instruction_chain);
            }
        }
    }

    println!("Sorting");
    let mut sum = 0;
    for p in &objects {
        let accepted = sort_part_to_accepted_or_not(&p, &chains);
        println!("Object {:?} => {} => {}", p, accepted, p.get_value());
        if accepted {
            sum += p.get_value();
        }
    }
    println!("Resulting part 1 sum {}", sum);

    let part_two = count_allowed_range(
        &chains,
        "in",
        std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>()), // seed with 1..4000 for all 4 slots
    );
    println!("Resulting part 2 sum {}", part_two);
}
