use std::fs::read_to_string;

use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// #[memoize(Capacity: 1_000_000)]

fn hash_inner(mut value: usize, input: char) -> usize {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    let val = input as usize;
    if input < ' ' {
        return value;
    }
    value += val;

    value *= 17;
    value = value % 256;

    value
}
fn hash_str(input: &str) -> usize {
    let mut sum = 0;

    for c in input.chars() {
        sum = hash_inner(sum, c);
    }
    sum
}
enum Action {
    Add,
    Remove,
}
#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}
fn lens_op_from_str(data: &str) -> (Lens, Action) {
    let sp_del: Vec<&str> = data.split('-').collect();
    let sp_add: Vec<&str> = data.split('=').collect();
    if sp_add.len() == 2 {
        return (
            Lens {
                label: sp_add[0].to_owned(),
                focal_length: sp_add[1].parse().unwrap(),
            },
            Action::Add,
        );
    }

    return (
        Lens {
            label: sp_del[0].to_owned(),
            focal_length: 0, //sp_del[1].parse().unwrap(),
        },
        Action::Remove,
    );
}

fn read_file(filename: &str) -> usize {
    let file_contents = read_to_string(filename).unwrap();
    let file_contents_clean = file_contents.trim();

    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for instruction in file_contents_clean.split(',') {
        let (lens, action) = lens_op_from_str(instruction);
        let box_num = hash_str(&lens.label);
        match action {
            Action::Add => {
                //
                // If there is already a lens in the box with the same label, replace the old lens with the new lens:
                //  remove the old lens and put the new lens in its place, not moving any other lenses in the box.
                if let Some(item_index) = boxes[box_num]
                    .iter()
                    .position(|len| len.label == lens.label)
                {
                    println!(
                        "Replace {:?} with {:?} in box {}",
                        boxes[box_num][item_index], lens, box_num
                    );
                    boxes[box_num][item_index].focal_length = lens.focal_length;
                    // Only attribute to update anyway
                } else {
                    // If there is not already a lens in the box with the same label,
                    //  add the lens to the box immediately behind any lenses already in the box. Don't move any of the other lenses when you do this. If there aren't any lenses in the box, the new lens goes all the way to the front of the box.
                    println!("Insert  {:?} in box {}", lens, box_num);
                    boxes[box_num].push(lens);
                }
            }
            Action::Remove => {
                //Go to box, if any lens has a matching label, remove it
                if let Some(item_index) = boxes[box_num]
                    .iter()
                    .position(|len| len.label == lens.label)
                {
                    println!("Remove  {:?} in box {}", lens, box_num);
                    boxes[box_num].remove(item_index);
                }
            }
        }
    }
    // Okay all lens should be in their box

    // To confirm that all of the lenses are installed correctly,
    // add up the focusing power of all of the lenses. The focusing power of a single lens is the result of multiplying together:

    //     One plus the box number of the lens in question.
    //     The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    //     The focal length of the lens.
    let mut sum = 0;
    for (box_num, lens) in boxes.iter().enumerate() {
        //
        for (lens_num, lens) in lens.iter().enumerate() {
            let value = (box_num + 1) * (lens_num + 1) * lens.focal_length;
            println!(
                "Box {} lens {} focus {:?} = {}",
                box_num, lens_num, lens, value
            );
            sum += value;
        }
    }
    sum
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
