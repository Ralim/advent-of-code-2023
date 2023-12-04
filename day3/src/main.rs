use std::{collections::HashSet, fs::read_to_string};

trait BoundedObject {
    fn get_bounding_coords(&self) -> (usize, usize, usize, usize);
    fn contains_coord(&self, x: usize, y: usize) -> bool;
    fn get_top_left(&self) -> (usize, usize);
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct NumberObject {
    number_value: u32,
    col: usize,
    row: usize,
    length: usize,
}
impl NumberObject {
    pub fn from_str(value: &str, col: usize, row: usize) -> Self {
        Self {
            col,
            row,
            length: value.len(),
            number_value: value.parse::<u32>().unwrap(),
        }
    }
    pub fn get_value(&self) -> u32 {
        return self.number_value;
    }
}
impl BoundedObject for NumberObject {
    fn get_bounding_coords(&self) -> (usize, usize, usize, usize) {
        return (self.col, self.row, self.length, 1);
    }

    fn get_top_left(&self) -> (usize, usize) {
        return (self.col, self.row);
    }

    fn contains_coord(&self, col: usize, row: usize) -> bool {
        let res = col >= self.col && col < (self.col + self.length) && row == self.row;
        if res {
            println!(
                "Hit {} ? col {} row {} <-> col {} row {} -> {}",
                self.get_value(),
                self.col,
                self.row,
                col,
                row,
                res
            );
        }
        return res;
    }
}
#[derive(Debug)]
struct SymbolObject {
    symbol: String,
    col: usize,
    row: usize,
    length: usize,
}
impl SymbolObject {
    pub fn from_str(value: &str, col: usize, row: usize) -> Self {
        Self {
            col,
            row,
            length: value.len(),
            symbol: value.to_owned(),
        }
    }
    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }
}

impl BoundedObject for SymbolObject {
    fn get_bounding_coords(&self) -> (usize, usize, usize, usize) {
        return (self.col, self.row, self.length, 1);
    }

    fn get_top_left(&self) -> (usize, usize) {
        return (self.col, self.row);
    }

    fn contains_coord(&self, x: usize, y: usize) -> bool {
        x >= self.col && x <= (self.col + self.length) && y == self.row
    }
}
#[derive(Debug)]
struct Schematic {
    width: usize,
    height: usize,
    numbers: Vec<NumberObject>,
    symbols: Vec<SymbolObject>,
}

impl Schematic {
    pub fn ingest_line(&mut self, line: &str, row: usize) {
        self.height = self.height.max(row); // Preserve max height
        self.width = self.width.max(line.len());
        let mut pushback = "".to_owned();
        let mut pushback_start = 0xFFFFFF;

        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                //Skip, blank filler
                if pushback.len() > 0 {
                    //We have a pushed back number, flush it out
                    let num = NumberObject::from_str(&pushback, pushback_start, row);
                    self.numbers.push(num);
                    pushback.clear();
                    pushback_start = 0xFFFFFF;
                }
            } else if char.is_ascii_digit() {
                pushback += &char.to_string();
                if pushback_start == 0xFFFFFF {
                    pushback_start = col;
                }
            } else {
                //This is a symbol (!num || !'.')
                if pushback.len() > 0 {
                    //We have a pushed back number, flush it out
                    let num = NumberObject::from_str(&pushback, pushback_start, row);
                    self.numbers.push(num);
                    pushback.clear();
                    pushback_start = 0xFFFFFF;
                }
                let symbol = SymbolObject::from_str(&char.to_string(), col, row);
                self.symbols.push(symbol);
            }
        }
        if pushback.len() > 0 {
            //We have a pushed back number, flush it out
            let num = NumberObject::from_str(&pushback, pushback_start, row);
            self.numbers.push(num);
            pushback.clear();
            pushback_start = 0xFFFFFF;
        }
    }
    fn get_bound_set(&self, object: &impl BoundedObject) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        let (x, y, width, height) = object.get_bounding_coords();
        let col_start = if x > 0 { x - 1 } else { x };
        let col_end = (self.width).min(x + width + 1);

        let row_start = if y > 0 { y - 1 } else { 0 };
        let row_end = (self.height + 1).min(y + height + 1);

        for xx in col_start..col_end {
            for yy in row_start..row_end {
                if !((xx == x) && (yy == y)) {
                    res.push((xx, yy));
                }
            }
        }
        res
    }
    pub fn select_numbers_near_symbols(&self) -> Vec<&NumberObject> {
        let mut selected_numbers = HashSet::new();

        for symbol in &self.symbols {
            //Filter criteria is that a number is valid iff it shares an edge to a symbol (diagonals count)
            // This means we can take the 3x3 bound of the symbol; if that intersects the number; its in
            let check_hit_boxes = self.get_bound_set(symbol);
            println!("Symbol {:?} Bounds -> {:?}", symbol, check_hit_boxes);
            for number in &self.numbers {
                for (x, y) in &check_hit_boxes {
                    if number.contains_coord(*x, *y) {
                        if selected_numbers.insert(number) {
                            println!("Inserted")
                        }
                        break;
                    }
                }
            }
        }

        selected_numbers.into_iter().collect()
    }
}
impl Default for Schematic {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            numbers: Vec::new(),
            symbols: Vec::new(),
        }
    }
}

fn read_file_to_schematic(filename: &str) -> Schematic {
    let mut schematic = Schematic::default();

    for (y, line) in read_to_string(filename).unwrap().lines().enumerate() {
        schematic.ingest_line(line, y);
    }
    schematic
}

fn main() {
    let schematic = read_file_to_schematic("input");
    // println!("{:?}", schematic);
    let hit_numbers = schematic.select_numbers_near_symbols();

    let mut sum = 0;
    for symbol in hit_numbers {
        println!("Hit Symbol {:?}", symbol);
        sum += symbol.get_value()
    }
    println!("Total sum {}", sum);
}
