use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, PartialEq, Eq)]

enum CardType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl CardType {
    pub fn get_rank(&self) -> usize {
        match self {
            CardType::FiveOfAKind => 7,
            CardType::FourOfAKind => 6,
            CardType::FullHouse => 5,
            CardType::ThreeOfAKind => 4,
            CardType::TwoPair => 3,
            CardType::OnePair => 2,
            CardType::HighCard => 1,
        }
    }
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}
impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    pub card_set: Vec<char>,
    distinct_cards: HashSet<char>,
    value: u64,
}
impl Hand {
    pub fn make(hand: &str, value: &str) -> Self {
        let chars: Vec<char> = hand.chars().collect();

        Self {
            card_set: chars.clone(),
            value: value.parse().unwrap(),
            distinct_cards: HashSet::from_iter(chars.iter().cloned()),
        }
    }
    pub fn get_value(&self) -> u64 {
        return self.value;
    }
    fn get_card_count(&self, card: &char) -> usize {
        let mut count = 0;
        for char in &self.card_set {
            if *char == *card {
                count += 1
            }
        }
        count
    }
    pub fn get_type(&self) -> CardType {
        if self.distinct_cards.len() == 1 {
            return CardType::FiveOfAKind;
        }
        if self.distinct_cards.len() == 2 {
            let set: Vec<&char> = self.distinct_cards.iter().collect();
            for c in &set {
                if self.get_card_count(c) == 4 {
                    return CardType::FourOfAKind;
                }
            }
            if self.get_card_count(set[0]) == 3 && self.get_card_count(set[1]) == 2 {
                return CardType::FullHouse;
            }
            if self.get_card_count(set[1]) == 3 && self.get_card_count(set[0]) == 2 {
                return CardType::FullHouse;
            }
        }
        if self.distinct_cards.len() == 3 {
            let set: Vec<&char> = self.distinct_cards.iter().collect();
            for c in &set {
                if self.get_card_count(c) == 3 {
                    return CardType::ThreeOfAKind;
                }
            }
            if self.get_card_count(set[0]) == 2 && self.get_card_count(set[1]) == 2 {
                return CardType::TwoPair;
            }
            if self.get_card_count(set[0]) == 2 && self.get_card_count(set[2]) == 2 {
                return CardType::TwoPair;
            }
            if self.get_card_count(set[1]) == 2 && self.get_card_count(set[2]) == 2 {
                return CardType::TwoPair;
            }
        }
        let set: Vec<&char> = self.distinct_cards.iter().collect();
        for c in set {
            if self.get_card_count(c) == 2 {
                return CardType::OnePair;
            }
        }
        CardType::HighCard
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type != other_type {
            return self_type.cmp(&other_type);
        }
        //if its the same type then we walk the chars in order
        for (c1, c2) in self.card_set.iter().zip(other.card_set.iter()) {
            if c1 != c2 {
                return card_to_virtual_rank(c1).cmp(&card_to_virtual_rank(c2));
            }
        }
        std::cmp::Ordering::Equal
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn card_to_virtual_rank(a: &char) -> usize {
    match a {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => panic!("Unhandled card {}", a),
    }
}

fn read_file(filename: &str) -> u64 {
    let mut hands = Vec::new();
    let file_contents = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    for line in lines {
        let sp: Vec<&str> = line.split(' ').collect();
        hands.push(Hand::make(sp[0], sp[1]));
    }
    //Sort hands lowest to highest
    hands.sort();
    for h in &hands {
        println!("{:?}->{:?}", h.card_set, h.get_type())
    }
    let mut sum = 0;
    for i in 0..hands.len() {
        sum += hands[i].get_value() * (i as u64 + 1)
    }
    sum
}

fn main() {
    let line_results = read_file("input");

    println!("Total {}", line_results);
}
