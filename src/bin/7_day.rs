use std::cmp;
use std::cmp::Ordering;
use std::fs;

#[repr(i16)]
#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum Trick {
    FiveKind(String) = 6,
    FourKind(String) = 5,
    FullHouse(String) = 4,
    ThreeKind(String) = 3,
    TwoPair(String) = 2,
    OnePair(String) = 1,
    HighCard(String) = 0,
}

static ORDER: &str = "23456789TJQKA";
fn custom_compare(a: &String, b: &String) -> Ordering {
    let a_char = a.chars().collect::<Vec<char>>();
    let b_char = b.chars().collect::<Vec<char>>();
    for i in 0..5 {
        let x = ORDER.find(a_char[i]);
        let y = ORDER.find(b_char[i]);
        if x < y {
            return Ordering::Less;
        } else if x > y {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

impl Ord for Trick {
    fn cmp(&self, other: &Self) -> Ordering {
        use Trick::*;
        match (self, other) {
            (FiveKind(a), FiveKind(b))
            | (FourKind(a), FourKind(b))
            | (FullHouse(a), FullHouse(b))
            | (ThreeKind(a), ThreeKind(b))
            | (TwoPair(a), TwoPair(b))
            | (OnePair(a), OnePair(b))
            | (HighCard(a), HighCard(b)) => custom_compare(a, b),
            _ => self.partial_cmp(other).unwrap(),
        }
    }
}

fn is_five_kind(s: &Vec<char>) -> bool {
    s[0] == s[1] && s[1] == s[2] && s[2] == s[3] && s[3] == s[4]
}

fn is_four_kind(s: &Vec<char>) -> bool {
    (s[0] == s[1] && s[1] == s[2] && s[2] == s[3]) || (s[1] == s[2] && s[2] == s[3] && s[3] == s[4])
}

fn is_full_house(s: &Vec<char>) -> bool {
    (s[0] == s[1] && s[2] == s[3] && s[3] == s[4]) || (s[0] == s[1] && s[1] == s[2] && s[3] == s[4])
}

fn is_three_kind(s: &Vec<char>) -> bool {
    (s[0] == s[1] && s[1] == s[2])
        || (s[1] == s[2] && s[2] == s[3])
        || (s[2] == s[3] && s[3] == s[4])
}

fn is_two_pair(s: &Vec<char>) -> bool {
    (s[0] == s[1] && s[2] == s[3])
        || (s[0] == s[1] && s[3] == s[4])
        || (s[1] == s[2] && s[3] == s[4])
}

fn is_one_pair(s: &Vec<char>) -> bool {
    (s[0] == s[1]) || (s[1] == s[2]) || (s[2] == s[3]) || (s[3] == s[4])
}

impl Trick {
    fn new(s: &str) -> Trick {
        let s_clone = &s;
        let mut sorted = s_clone.chars().collect::<Vec<char>>();
        sorted.sort();
        let s_string = s.to_string();
        if is_five_kind(&sorted) {
            Trick::FiveKind(s_string)
        } else if is_four_kind(&sorted) {
            Trick::FourKind(s_string)
        } else if is_full_house(&sorted) {
            Trick::FullHouse(s_string)
        } else if is_three_kind(&sorted) {
            Trick::ThreeKind(s_string)
        } else if is_two_pair(&sorted) {
            Trick::TwoPair(s_string)
        } else if is_one_pair(&sorted) {
            Trick::OnePair(s_string)
        } else {
            Trick::HighCard(s_string)
        }
    }

    fn joker(s: &str) -> Trick {
        let s_clone = &s;
        let mut res : Trick = Trick::HighCard(String::from("22222"));
        for i in ORDER.chars().skip(1) {
            let replaced = s_clone.replace("J", i.to_string().as_str());
            let mut sorted = replaced.chars().collect::<Vec<char>>();
            sorted.sort();
            let s_string = s.to_string();
            if is_five_kind(&sorted) {
                res = cmp::max(res, Trick::FiveKind(s_string));
            } else if is_four_kind(&sorted) {
                res = cmp::max(res, Trick::FourKind(s_string));
            } else if is_full_house(&sorted) {
                res = cmp::max(res, Trick::FullHouse(s_string));
            } else if is_three_kind(&sorted) {
                res = cmp::max(res, Trick::ThreeKind(s_string));
            } else if is_two_pair(&sorted) {
                res = cmp::max(res, Trick::TwoPair(s_string));
            } else if is_one_pair(&sorted) {
                res = cmp::max(res, Trick::OnePair(s_string));
            } else {
                res = cmp::max(res, Trick::HighCard(s_string));
            }
        }

        res
    }
}

struct Hand {
    trick: Trick,
    bid: i32,
}

impl Hand {
    fn new(s: &str) -> Hand {
        let mut sep = s.split_whitespace();
        Hand {
            trick: Trick::new(sep.nth(0).unwrap()),
            bid: sep.nth(0).unwrap().parse::<i32>().unwrap(),
        }
    }

    fn joker(s: &str) -> Hand {
        let mut sep = s.split_whitespace();
        Hand {
            trick: Trick::joker(sep.nth(0).unwrap()),
            bid: sep.nth(0).unwrap().parse::<i32>().unwrap(),
        }
    }
}
fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("[Error] Could not read file");

    let mut hands: Vec<Hand> = Vec::new();

    for line in contents.lines() {
        hands.push(Hand::new(line));
    }
    hands.sort_by(|a, b| (&a.trick).cmp(&b.trick));
    let mut part1ans = 0;
    for (id, h) in hands.iter().enumerate() {
        part1ans += h.bid * (id + 1) as i32;
    }

    println!("Part 1 ans => {part1ans}");

    hands.clear();
    for line in contents.lines() {
        hands.push(Hand::joker(line));
    }
    hands.sort_by(|a, b| (&a.trick).cmp(&b.trick));
    let mut part2ans = 0;
    for (id, h) in hands.iter().enumerate() {
        part2ans += h.bid * (id + 1) as i32;
    }

    println!("Part 2 ans => {part2ans}");
}
