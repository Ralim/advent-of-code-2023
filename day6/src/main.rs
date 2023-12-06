use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct TimedRace {
    time: u64,
    distance: u64,
}
impl TimedRace {
    pub fn make(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }
    pub fn get_winning_button_times(&self) -> Vec<u64> {
        let mut res = Vec::new();
        for hold_time in 1..self.time - 1 {
            let speed = hold_time;
            let time_left = self.time - hold_time;
            let distance = time_left * speed;
            if distance > self.distance {
                res.push(hold_time);
            }
        }
        res
    }
}

fn string_to_split_nums(chunk_in: &str) -> Vec<u64> {
    let mut res = Vec::new();
    for s in chunk_in.split(' ') {
        let safe = s.trim();
        if safe.len() > 0 {
            res.push(s.parse().unwrap());
        }
    }
    res
}

fn read_file(filename: &str) -> u64 {
    let mut races: Vec<TimedRace> = Vec::new();
    {
        let file_contents = read_to_string(filename).unwrap();
        let lines: Vec<&str> = file_contents.lines().collect();
        let times = string_to_split_nums(lines[0].split(':').collect::<Vec<&str>>()[1]);
        let distances = string_to_split_nums(lines[1].split(':').collect::<Vec<&str>>()[1]);
        for (time, dist) in times.iter().zip(distances.iter()) {
            let race = TimedRace::make(*time, *dist);
            races.push(race);
        }
    }
    //Have the races, now need to solve potentials
    let mut possible_win_combos = 1;
    for race in races {
        let winning_holds = race.get_winning_button_times();
        // println!("Race Winners {:?}", winning_holds);
        possible_win_combos *= winning_holds.len();
    }
    possible_win_combos as u64
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
